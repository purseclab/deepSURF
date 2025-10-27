use crate::core::dependency::Dependency;
use crate::core::registry::PackageRegistry;
use crate::core::resolver::features::{CliFeatures, HasDevUnits};
use crate::core::shell::Verbosity;
use crate::core::Registry as _;
use crate::core::{PackageId, PackageIdSpec, PackageIdSpecQuery};
use crate::core::{Resolve, SourceId, Workspace};
use crate::ops;
use crate::sources::source::QueryKind;
use crate::util::cache_lock::CacheLockMode;
use crate::util::context::GlobalContext;
use crate::util::toml_mut::dependency::{MaybeWorkspace, Source};
use crate::util::toml_mut::manifest::LocalManifest;
use crate::util::toml_mut::upgrade::upgrade_requirement;
use crate::util::{style, OptVersionReq};
use crate::util::{CargoResult, VersionExt};
use itertools::Itertools;
use semver::{Op, Version, VersionReq};
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap, HashSet};
use tracing::{debug, trace};

pub type UpgradeMap = HashMap<(String, SourceId), Version>;

pub struct UpdateOptions<'a> {
    pub gctx: &'a GlobalContext,
    pub to_update: Vec<String>,
    pub precise: Option<&'a str>,
    pub recursive: bool,
    pub dry_run: bool,
    pub workspace: bool,
}

pub fn generate_lockfile(ws: &Workspace<'_>) -> CargoResult<()> {
    let mut registry = ws.package_registry()?;
    let previous_resolve = None;
    let mut resolve = ops::resolve_with_previous(
        &mut registry,
        ws,
        &CliFeatures::new_all(true),
        HasDevUnits::Yes,
        previous_resolve,
        None,
        &[],
        true,
    )?;
    ops::write_pkg_lockfile(ws, &mut resolve)?;
    print_lockfile_changes(ws, previous_resolve, &resolve, &mut registry)?;
    Ok(())
}

pub fn update_lockfile(ws: &Workspace<'_>, opts: &UpdateOptions<'_>) -> CargoResult<()> {
    if opts.recursive && opts.precise.is_some() {
        anyhow::bail!("cannot specify both recursive and precise simultaneously")
    }

    if ws.members().count() == 0 {
        anyhow::bail!("you can't generate a lockfile for an empty workspace.")
    }

    // Updates often require a lot of modifications to the registry, so ensure
    // that we're synchronized against other Cargos.
    let _lock = ws
        .gctx()
        .acquire_package_cache_lock(CacheLockMode::DownloadExclusive)?;

    let previous_resolve = match ops::load_pkg_lockfile(ws)? {
        Some(resolve) => resolve,
        None => {
            match opts.precise {
                None => return generate_lockfile(ws),

                // Precise option specified, so calculate a previous_resolve required
                // by precise package update later.
                Some(_) => {
                    let mut registry = ws.package_registry()?;
                    ops::resolve_with_previous(
                        &mut registry,
                        ws,
                        &CliFeatures::new_all(true),
                        HasDevUnits::Yes,
                        None,
                        None,
                        &[],
                        true,
                    )?
                }
            }
        }
    };
    let mut registry = ws.package_registry()?;
    let mut to_avoid = HashSet::new();

    if opts.to_update.is_empty() {
        if !opts.workspace {
            to_avoid.extend(previous_resolve.iter());
            to_avoid.extend(previous_resolve.unused_patches());
        }
    } else {
        let mut sources = Vec::new();
        for name in opts.to_update.iter() {
            let pid = previous_resolve.query(name)?;
            if opts.recursive {
                fill_with_deps(&previous_resolve, pid, &mut to_avoid, &mut HashSet::new());
            } else {
                to_avoid.insert(pid);
                sources.push(match opts.precise {
                    Some(precise) => {
                        // TODO: see comment in `resolve.rs` as well, but this
                        //       seems like a pretty hokey reason to single out
                        //       the registry as well.
                        if pid.source_id().is_registry() {
                            pid.source_id().with_precise_registry_version(
                                pid.name(),
                                pid.version().clone(),
                                precise,
                            )?
                        } else {
                            pid.source_id().with_git_precise(Some(precise.to_string()))
                        }
                    }
                    None => pid.source_id().without_precise(),
                });
            }
            if let Ok(unused_id) =
                PackageIdSpec::query_str(name, previous_resolve.unused_patches().iter().cloned())
            {
                to_avoid.insert(unused_id);
            }
        }

        // Mirror `--workspace` and never avoid workspace members.
        // Filtering them out here so the above processes them normally
        // so their dependencies can be updated as requested
        to_avoid = to_avoid
            .into_iter()
            .filter(|id| {
                for package in ws.members() {
                    let member_id = package.package_id();
                    // Skip checking the `version` because `previous_resolve` might have a stale
                    // value.
                    // When dealing with workspace members, the other fields should be a
                    // sufficiently unique match.
                    if id.name() == member_id.name() && id.source_id() == member_id.source_id() {
                        return false;
                    }
                }
                true
            })
            .collect();

        registry.add_sources(sources)?;
    }

    // Here we place an artificial limitation that all non-registry sources
    // cannot be locked at more than one revision. This means that if a Git
    // repository provides more than one package, they must all be updated in
    // step when any of them are updated.
    //
    // TODO: this seems like a hokey reason to single out the registry as being
    // different.
    let to_avoid_sources: HashSet<_> = to_avoid
        .iter()
        .map(|p| p.source_id())
        .filter(|s| !s.is_registry())
        .collect();

    let keep = |p: &PackageId| !to_avoid_sources.contains(&p.source_id()) && !to_avoid.contains(p);

    let mut resolve = ops::resolve_with_previous(
        &mut registry,
        ws,
        &CliFeatures::new_all(true),
        HasDevUnits::Yes,
        Some(&previous_resolve),
        Some(&keep),
        &[],
        true,
    )?;

    print_lockfile_updates(
        ws,
        &previous_resolve,
        &resolve,
        opts.precise.is_some(),
        &mut registry,
    )?;
    if opts.dry_run {
        opts.gctx
            .shell()
            .warn("not updating lockfile due to dry run")?;
    } else {
        ops::write_pkg_lockfile(ws, &mut resolve)?;
    }
    Ok(())
}

/// Prints lockfile change statuses.
///
/// This would acquire the package-cache lock, as it may update the index to
/// show users latest available versions.
pub fn print_lockfile_changes(
    ws: &Workspace<'_>,
    previous_resolve: Option<&Resolve>,
    resolve: &Resolve,
    registry: &mut PackageRegistry<'_>,
) -> CargoResult<()> {
    let _lock = ws
        .gctx()
        .acquire_package_cache_lock(CacheLockMode::DownloadExclusive)?;
    if let Some(previous_resolve) = previous_resolve {
        print_lockfile_sync(ws, previous_resolve, resolve, registry)
    } else {
        print_lockfile_generation(ws, resolve, registry)
    }
}
pub fn upgrade_manifests(
    ws: &mut Workspace<'_>,
    to_update: &Vec<String>,
) -> CargoResult<UpgradeMap> {
    let gctx = ws.gctx();
    let mut upgrades = HashMap::new();
    let mut upgrade_messages = HashSet::new();

    // Updates often require a lot of modifications to the registry, so ensure
    // that we're synchronized against other Cargos.
    let _lock = gctx.acquire_package_cache_lock(CacheLockMode::DownloadExclusive)?;

    let mut registry = ws.package_registry()?;
    registry.lock_patches();

    for member in ws.members_mut().sorted() {
        debug!("upgrading manifest for `{}`", member.name());

        *member.manifest_mut().summary_mut() = member
            .manifest()
            .summary()
            .clone()
            .try_map_dependencies(|d| {
                upgrade_dependency(
                    &gctx,
                    to_update,
                    &mut registry,
                    &mut upgrades,
                    &mut upgrade_messages,
                    d,
                )
            })?;
    }

    Ok(upgrades)
}

fn upgrade_dependency(
    gctx: &GlobalContext,
    to_update: &Vec<String>,
    registry: &mut PackageRegistry<'_>,
    upgrades: &mut UpgradeMap,
    upgrade_messages: &mut HashSet<String>,
    dependency: Dependency,
) -> CargoResult<Dependency> {
    let name = dependency.package_name();
    let renamed_to = dependency.name_in_toml();

    if name != renamed_to {
        trace!(
            "skipping dependency renamed from `{}` to `{}`",
            name,
            renamed_to
        );
        return Ok(dependency);
    }

    if !to_update.is_empty() && !to_update.contains(&name.to_string()) {
        trace!("skipping dependency `{}` not selected for upgrading", name);
        return Ok(dependency);
    }

    if !dependency.source_id().is_registry() {
        trace!("skipping non-registry dependency: {}", name);
        return Ok(dependency);
    }

    let version_req = dependency.version_req();

    let OptVersionReq::Req(current) = version_req else {
        trace!(
            "skipping dependency `{}` without a simple version requirement: {}",
            name,
            version_req
        );
        return Ok(dependency);
    };

    let [comparator] = &current.comparators[..] else {
        trace!(
            "skipping dependency `{}` with multiple version comparators: {:?}",
            name,
            &current.comparators
        );
        return Ok(dependency);
    };

    if comparator.op != Op::Caret {
        trace!("skipping non-caret dependency `{}`: {}", name, comparator);
        return Ok(dependency);
    }

    let query =
        crate::core::dependency::Dependency::parse(name, None, dependency.source_id().clone())?;

    let possibilities = {
        loop {
            match registry.query_vec(&query, QueryKind::Exact) {
                std::task::Poll::Ready(res) => {
                    break res?;
                }
                std::task::Poll::Pending => registry.block_until_ready()?,
            }
        }
    };

    let latest = if !possibilities.is_empty() {
        possibilities
            .iter()
            .map(|s| s.as_summary())
            .map(|s| s.version())
            .filter(|v| !v.is_prerelease())
            .max()
    } else {
        None
    };

    let Some(latest) = latest else {
        trace!(
            "skipping dependency `{}` without any published versions",
            name
        );
        return Ok(dependency);
    };

    if current.matches(&latest) {
        trace!(
            "skipping dependency `{}` without a breaking update available",
            name
        );
        return Ok(dependency);
    }

    let Some(new_req_string) = upgrade_requirement(&current.to_string(), latest)? else {
        trace!(
            "skipping dependency `{}` because the version requirement didn't change",
            name
        );
        return Ok(dependency);
    };

    let upgrade_message = format!("{} {} -> {}", name, current, new_req_string);
    trace!(upgrade_message);

    if upgrade_messages.insert(upgrade_message.clone()) {
        gctx.shell()
            .status_with_color("Upgrading", &upgrade_message, &style::GOOD)?;
    }

    upgrades.insert((name.to_string(), dependency.source_id()), latest.clone());

    let req = OptVersionReq::Req(VersionReq::parse(&latest.to_string())?);
    let mut dep = dependency.clone();
    dep.set_version_req(req);
    Ok(dep)
}

/// Update manifests with upgraded versions, and write to disk. Based on cargo-edit.
/// Returns true if any file has changed.
pub fn write_manifest_upgrades(
    ws: &Workspace<'_>,
    upgrades: &UpgradeMap,
    dry_run: bool,
) -> CargoResult<bool> {
    if upgrades.is_empty() {
        return Ok(false);
    }

    let mut any_file_has_changed = false;

    let manifest_paths = std::iter::once(ws.root_manifest())
        .chain(ws.members().map(|member| member.manifest_path()))
        .collect::<Vec<_>>();

    for manifest_path in manifest_paths {
        trace!(
            "updating TOML manifest at `{:?}` with upgraded dependencies",
            manifest_path
        );

        let crate_root = manifest_path
            .parent()
            .expect("manifest path is absolute")
            .to_owned();

        let mut local_manifest = LocalManifest::try_new(&manifest_path)?;
        let mut manifest_has_changed = false;

        for dep_table in local_manifest.get_dependency_tables_mut() {
            for (mut dep_key, dep_item) in dep_table.iter_mut() {
                let dep_key_str = dep_key.get();
                let dependency = crate::util::toml_mut::dependency::Dependency::from_toml(
                    &manifest_path,
                    dep_key_str,
                    dep_item,
                )?;

                let Some(current) = dependency.version() else {
                    trace!("skipping dependency without a version: {}", dependency.name);
                    continue;
                };

                let (MaybeWorkspace::Other(source_id), Some(Source::Registry(source))) =
                    (dependency.source_id(ws.gctx())?, dependency.source())
                else {
                    trace!("skipping non-registry dependency: {}", dependency.name);
                    continue;
                };

                let Some(latest) = upgrades.get(&(dependency.name.to_owned(), source_id)) else {
                    trace!(
                        "skipping dependency without an upgrade: {}",
                        dependency.name
                    );
                    continue;
                };

                let Some(new_req_string) = upgrade_requirement(current, latest)? else {
                    trace!(
                        "skipping dependency `{}` because the version requirement didn't change",
                        dependency.name
                    );
                    continue;
                };

                let mut dep = dependency.clone();
                let mut source = source.clone();
                source.version = new_req_string;
                dep.source = Some(Source::Registry(source));

                trace!("upgrading dependency {}", dependency.name);
                dep.update_toml(&crate_root, &mut dep_key, dep_item);
                manifest_has_changed = true;
                any_file_has_changed = true;
            }
        }

        if manifest_has_changed && !dry_run {
            debug!("writing upgraded manifest to {}", manifest_path.display());
            local_manifest.write()?;
        }
    }

    Ok(any_file_has_changed)
}

fn print_lockfile_generation(
    ws: &Workspace<'_>,
    resolve: &Resolve,
    registry: &mut PackageRegistry<'_>,
) -> CargoResult<()> {
    let diff = PackageDiff::new(&resolve);
    let num_pkgs: usize = diff.iter().map(|d| d.added.len()).sum();
    if num_pkgs <= 1 {
        // just ourself, nothing worth reporting
        return Ok(());
    }
    status_locking(ws, num_pkgs)?;

    for diff in diff {
        fn format_latest(version: semver::Version) -> String {
            let warn = style::WARN;
            format!(" {warn}(latest: v{version}){warn:#}")
        }
        let possibilities = if let Some(query) = diff.alternatives_query() {
            loop {
                match registry.query_vec(&query, QueryKind::Exact) {
                    std::task::Poll::Ready(res) => {
                        break res?;
                    }
                    std::task::Poll::Pending => registry.block_until_ready()?,
                }
            }
        } else {
            vec![]
        };

        for package in diff.added.iter() {
            let latest = if !possibilities.is_empty() {
                possibilities
                    .iter()
                    .map(|s| s.as_summary())
                    .filter(|s| is_latest(s.version(), package.version()))
                    .map(|s| s.version().clone())
                    .max()
                    .map(format_latest)
            } else {
                None
            };

            if let Some(latest) = latest {
                ws.gctx().shell().status_with_color(
                    "Adding",
                    format!("{package}{latest}"),
                    &style::NOTE,
                )?;
            }
        }
    }

    Ok(())
}

fn print_lockfile_sync(
    ws: &Workspace<'_>,
    previous_resolve: &Resolve,
    resolve: &Resolve,
    registry: &mut PackageRegistry<'_>,
) -> CargoResult<()> {
    let diff = PackageDiff::diff(&previous_resolve, &resolve);
    let num_pkgs: usize = diff.iter().map(|d| d.added.len()).sum();
    if num_pkgs == 0 {
        return Ok(());
    }
    status_locking(ws, num_pkgs)?;

    for diff in diff {
        fn format_latest(version: semver::Version) -> String {
            let warn = style::WARN;
            format!(" {warn}(latest: v{version}){warn:#}")
        }
        let possibilities = if let Some(query) = diff.alternatives_query() {
            loop {
                match registry.query_vec(&query, QueryKind::Exact) {
                    std::task::Poll::Ready(res) => {
                        break res?;
                    }
                    std::task::Poll::Pending => registry.block_until_ready()?,
                }
            }
        } else {
            vec![]
        };

        if let Some((removed, added)) = diff.change() {
            let latest = if !possibilities.is_empty() {
                possibilities
                    .iter()
                    .map(|s| s.as_summary())
                    .filter(|s| is_latest(s.version(), added.version()))
                    .map(|s| s.version().clone())
                    .max()
                    .map(format_latest)
            } else {
                None
            }
            .unwrap_or_default();

            let msg = if removed.source_id().is_git() {
                format!(
                    "{removed} -> #{}",
                    &added.source_id().precise_git_fragment().unwrap()[..8],
                )
            } else {
                format!("{removed} -> v{}{latest}", added.version())
            };

            // If versions differ only in build metadata, we call it an "update"
            // regardless of whether the build metadata has gone up or down.
            // This metadata is often stuff like git commit hashes, which are
            // not meaningfully ordered.
            if removed.version().cmp_precedence(added.version()) == Ordering::Greater {
                ws.gctx()
                    .shell()
                    .status_with_color("Downgrading", msg, &style::WARN)?;
            } else {
                ws.gctx()
                    .shell()
                    .status_with_color("Updating", msg, &style::GOOD)?;
            }
        } else {
            for package in diff.added.iter() {
                let latest = if !possibilities.is_empty() {
                    possibilities
                        .iter()
                        .map(|s| s.as_summary())
                        .filter(|s| is_latest(s.version(), package.version()))
                        .map(|s| s.version().clone())
                        .max()
                        .map(format_latest)
                } else {
                    None
                }
                .unwrap_or_default();

                ws.gctx().shell().status_with_color(
                    "Adding",
                    format!("{package}{latest}"),
                    &style::NOTE,
                )?;
            }
        }
    }

    Ok(())
}

fn print_lockfile_updates(
    ws: &Workspace<'_>,
    previous_resolve: &Resolve,
    resolve: &Resolve,
    precise: bool,
    registry: &mut PackageRegistry<'_>,
) -> CargoResult<()> {
    let diff = PackageDiff::diff(&previous_resolve, &resolve);
    let num_pkgs: usize = diff.iter().map(|d| d.added.len()).sum();
    if !precise {
        status_locking(ws, num_pkgs)?;
    }

    let mut unchanged_behind = 0;
    for diff in diff {
        fn format_latest(version: semver::Version) -> String {
            let warn = style::WARN;
            format!(" {warn}(latest: v{version}){warn:#}")
        }
        let possibilities = if let Some(query) = diff.alternatives_query() {
            loop {
                match registry.query_vec(&query, QueryKind::Exact) {
                    std::task::Poll::Ready(res) => {
                        break res?;
                    }
                    std::task::Poll::Pending => registry.block_until_ready()?,
                }
            }
        } else {
            vec![]
        };

        if let Some((removed, added)) = diff.change() {
            let latest = if !possibilities.is_empty() {
                possibilities
                    .iter()
                    .map(|s| s.as_summary())
                    .filter(|s| is_latest(s.version(), added.version()))
                    .map(|s| s.version().clone())
                    .max()
                    .map(format_latest)
            } else {
                None
            }
            .unwrap_or_default();

            let msg = if removed.source_id().is_git() {
                format!(
                    "{removed} -> #{}",
                    &added.source_id().precise_git_fragment().unwrap()[..8],
                )
            } else {
                format!("{removed} -> v{}{latest}", added.version())
            };

            // If versions differ only in build metadata, we call it an "update"
            // regardless of whether the build metadata has gone up or down.
            // This metadata is often stuff like git commit hashes, which are
            // not meaningfully ordered.
            if removed.version().cmp_precedence(added.version()) == Ordering::Greater {
                ws.gctx()
                    .shell()
                    .status_with_color("Downgrading", msg, &style::WARN)?;
            } else {
                ws.gctx()
                    .shell()
                    .status_with_color("Updating", msg, &style::GOOD)?;
            }
        } else {
            for package in diff.removed.iter() {
                ws.gctx().shell().status_with_color(
                    "Removing",
                    format!("{package}"),
                    &style::ERROR,
                )?;
            }
            for package in diff.added.iter() {
                let latest = if !possibilities.is_empty() {
                    possibilities
                        .iter()
                        .map(|s| s.as_summary())
                        .filter(|s| is_latest(s.version(), package.version()))
                        .map(|s| s.version().clone())
                        .max()
                        .map(format_latest)
                } else {
                    None
                }
                .unwrap_or_default();

                ws.gctx().shell().status_with_color(
                    "Adding",
                    format!("{package}{latest}"),
                    &style::NOTE,
                )?;
            }
        }
        for package in &diff.unchanged {
            let latest = if !possibilities.is_empty() {
                possibilities
                    .iter()
                    .map(|s| s.as_summary())
                    .filter(|s| is_latest(s.version(), package.version()))
                    .map(|s| s.version().clone())
                    .max()
                    .map(format_latest)
            } else {
                None
            };

            if let Some(latest) = latest {
                unchanged_behind += 1;
                if ws.gctx().shell().verbosity() == Verbosity::Verbose {
                    ws.gctx().shell().status_with_color(
                        "Unchanged",
                        format!("{package}{latest}"),
                        &anstyle::Style::new().bold(),
                    )?;
                }
            }
        }
    }

    if ws.gctx().shell().verbosity() == Verbosity::Verbose {
        ws.gctx().shell().note(
            "to see how you depend on a package, run `cargo tree --invert --package <dep>@<ver>`",
        )?;
    } else {
        if 0 < unchanged_behind {
            ws.gctx().shell().note(format!(
                "pass `--verbose` to see {unchanged_behind} unchanged dependencies behind latest"
            ))?;
        }
    }

    Ok(())
}

fn status_locking(ws: &Workspace<'_>, num_pkgs: usize) -> CargoResult<()> {
    use std::fmt::Write as _;

    let plural = if num_pkgs == 1 { "" } else { "s" };

    let mut cfg = String::new();
    // Don't have a good way to describe `direct_minimal_versions` atm
    if !ws.gctx().cli_unstable().direct_minimal_versions {
        write!(&mut cfg, " to")?;
        if ws.gctx().cli_unstable().minimal_versions {
            write!(&mut cfg, " earliest")?;
        } else {
            write!(&mut cfg, " latest")?;
        }

        if ws.resolve_honors_rust_version() {
            let rust_version = if let Some(ver) = ws.rust_version() {
                ver.clone().into_partial()
            } else {
                let rustc = ws.gctx().load_global_rustc(Some(ws))?;
                let rustc_version = rustc.version.clone().into();
                rustc_version
            };
            write!(&mut cfg, " Rust {rust_version}")?;
        }
        write!(&mut cfg, " compatible version{plural}")?;
    }

    ws.gctx()
        .shell()
        .status("Locking", format!("{num_pkgs} package{plural}{cfg}"))?;
    Ok(())
}

fn is_latest(candidate: &semver::Version, current: &semver::Version) -> bool {
    current < candidate
                // Only match pre-release if major.minor.patch are the same
                && (candidate.pre.is_empty()
                    || (candidate.major == current.major
                        && candidate.minor == current.minor
                        && candidate.patch == current.patch))
}

fn fill_with_deps<'a>(
    resolve: &'a Resolve,
    dep: PackageId,
    set: &mut HashSet<PackageId>,
    visited: &mut HashSet<PackageId>,
) {
    if !visited.insert(dep) {
        return;
    }
    set.insert(dep);
    for (dep, _) in resolve.deps_not_replaced(dep) {
        fill_with_deps(resolve, dep, set, visited);
    }
}

/// All resolved versions of a package name within a [`SourceId`]
#[derive(Default, Clone, Debug)]
pub struct PackageDiff {
    removed: Vec<PackageId>,
    added: Vec<PackageId>,
    unchanged: Vec<PackageId>,
}

impl PackageDiff {
    pub fn new(resolve: &Resolve) -> Vec<Self> {
        let mut changes = BTreeMap::new();
        let empty = Self::default();
        for dep in resolve.iter() {
            changes
                .entry(Self::key(dep))
                .or_insert_with(|| empty.clone())
                .added
                .push(dep);
        }

        changes.into_iter().map(|(_, v)| v).collect()
    }

    pub fn diff(previous_resolve: &Resolve, resolve: &Resolve) -> Vec<Self> {
        fn vec_subset(a: &[PackageId], b: &[PackageId]) -> Vec<PackageId> {
            a.iter().filter(|a| !contains_id(b, a)).cloned().collect()
        }

        fn vec_intersection(a: &[PackageId], b: &[PackageId]) -> Vec<PackageId> {
            a.iter().filter(|a| contains_id(b, a)).cloned().collect()
        }

        // Check if a PackageId is present `b` from `a`.
        //
        // Note that this is somewhat more complicated because the equality for source IDs does not
        // take precise versions into account (e.g., git shas), but we want to take that into
        // account here.
        fn contains_id(haystack: &[PackageId], needle: &PackageId) -> bool {
            let Ok(i) = haystack.binary_search(needle) else {
                return false;
            };

            // If we've found `a` in `b`, then we iterate over all instances
            // (we know `b` is sorted) and see if they all have different
            // precise versions. If so, then `a` isn't actually in `b` so
            // we'll let it through.
            //
            // Note that we only check this for non-registry sources,
            // however, as registries contain enough version information in
            // the package ID to disambiguate.
            if needle.source_id().is_registry() {
                return true;
            }
            haystack[i..]
                .iter()
                .take_while(|b| &needle == b)
                .any(|b| needle.source_id().has_same_precise_as(b.source_id()))
        }

        // Map `(package name, package source)` to `(removed versions, added versions)`.
        let mut changes = BTreeMap::new();
        let empty = Self::default();
        for dep in previous_resolve.iter() {
            changes
                .entry(Self::key(dep))
                .or_insert_with(|| empty.clone())
                .removed
                .push(dep);
        }
        for dep in resolve.iter() {
            changes
                .entry(Self::key(dep))
                .or_insert_with(|| empty.clone())
                .added
                .push(dep);
        }

        for v in changes.values_mut() {
            let Self {
                removed: ref mut old,
                added: ref mut new,
                unchanged: ref mut other,
            } = *v;
            old.sort();
            new.sort();
            let removed = vec_subset(old, new);
            let added = vec_subset(new, old);
            let unchanged = vec_intersection(new, old);
            *old = removed;
            *new = added;
            *other = unchanged;
        }
        debug!("{:#?}", changes);

        changes.into_iter().map(|(_, v)| v).collect()
    }

    fn key(dep: PackageId) -> (&'static str, SourceId) {
        (dep.name().as_str(), dep.source_id())
    }

    /// Guess if a package upgraded/downgraded
    ///
    /// All `PackageDiff` knows is that entries were added/removed within [`Resolve`].
    /// A package could be added or removed because of dependencies from other packages
    /// which makes it hard to definitively say "X was upgrade to N".
    pub fn change(&self) -> Option<(&PackageId, &PackageId)> {
        if self.removed.len() == 1 && self.added.len() == 1 {
            Some((&self.removed[0], &self.added[0]))
        } else {
            None
        }
    }

    /// For querying [`PackageRegistry`] for alternative versions to report to the user
    pub fn alternatives_query(&self) -> Option<crate::core::dependency::Dependency> {
        let package_id = [
            self.added.iter(),
            self.unchanged.iter(),
            self.removed.iter(),
        ]
        .into_iter()
        .flatten()
        .next()
        // Limit to registry as that is the only source with meaningful alternative versions
        .filter(|s| s.source_id().is_registry())?;
        let query = crate::core::dependency::Dependency::parse(
            package_id.name(),
            None,
            package_id.source_id(),
        )
        .expect("already a valid dependency");
        Some(query)
    }
}
