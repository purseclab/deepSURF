//! Tests for targets with `rust-version`.

#![allow(deprecated)]

use cargo_test_support::{cargo_process, project, registry::Package};

#[cargo_test]
fn rust_version_satisfied() {
    let p = project()
        .file(
            "Cargo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.0.1"
            edition = "2015"
            authors = []
            rust-version = "1.1.1"
            [[bin]]
            name = "foo"
        "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    p.cargo("check").run();
    p.cargo("check --ignore-rust-version").run();
}

#[cargo_test]
fn rust_version_error() {
    project()
        .file(
            "Cargo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.0.1"
            edition = "2015"
            authors = []
            rust-version = "^1.43"
            [[bin]]
            name = "foo"
        "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build()
        .cargo("check")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] unexpected version requirement, expected a version like \"1.32\"
 --> Cargo.toml:7:28
  |
7 |             rust-version = \"^1.43\"
  |                            ^^^^^^^
  |
",
        )
        .run();
}

#[cargo_test]
fn rust_version_older_than_edition() {
    project()
        .file(
            "Cargo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.0.1"
            authors = []
            rust-version = "1.1"
            edition = "2018"
            [[bin]]
            name = "foo"
        "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build()
        .cargo("check")
        .with_status(101)
        .with_stderr_contains("  rust-version 1.1 is older than first version (1.31.0) required by the specified edition (2018)",
        )
        .run();
}

#[cargo_test]
fn lint_self_incompatible_with_rust_version() {
    let p = project()
        .file(
            "Cargo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.0.1"
            edition = "2015"
            authors = []
            rust-version = "1.9876.0"
            [[bin]]
            name = "foo"
        "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    p.cargo("check")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] rustc [..] is not supported by the following package:
  foo@0.0.1 requires rustc 1.9876.0

",
        )
        .run();
    p.cargo("check --ignore-rust-version").run();
}

#[cargo_test]
fn lint_dep_incompatible_with_rust_version() {
    Package::new("too_new_parent", "0.0.1")
        .dep("too_new_child", "0.0.1")
        .rust_version("1.2345.0")
        .file("src/lib.rs", "fn other_stuff() {}")
        .publish();
    Package::new("too_new_child", "0.0.1")
        .rust_version("1.2345.0")
        .file("src/lib.rs", "fn other_stuff() {}")
        .publish();
    Package::new("rustc_compatible", "0.0.1")
        .rust_version("1.60.0")
        .file("src/lib.rs", "fn other_stuff() {}")
        .publish();

    let p = project()
        .file(
            "Cargo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.0.1"
            edition = "2015"
            rust-version = "1.50"
            authors = []
            [dependencies]
            too_new_parent = "0.0.1"
            rustc_compatible = "0.0.1"
        "#,
        )
        .file("src/main.rs", "fn main(){}")
        .build();

    p.cargo("generate-lockfile")
        .with_stderr(
            "\
[UPDATING] `[..]` index
[LOCKING] 4 packages to latest compatible versions
",
        )
        .run();
    p.cargo("check")
        .with_status(101)
        .with_stderr(
            "\
[DOWNLOADING] crates ...
[DOWNLOADED] too_new_parent v0.0.1 (registry `[..]`)
[DOWNLOADED] too_new_child v0.0.1 (registry `[..]`)
[DOWNLOADED] rustc_compatible v0.0.1 (registry `[..]`)
[ERROR] rustc [..] is not supported by the following packages:
  too_new_child@0.0.1 requires rustc 1.2345.0
  too_new_parent@0.0.1 requires rustc 1.2345.0
Either upgrade rustc or select compatible dependency versions with
`cargo update <name>@<current-ver> --precise <compatible-ver>`
where `<compatible-ver>` is the latest version supporting rustc [..]

",
        )
        .run();
    p.cargo("check --ignore-rust-version").run();
}

#[cargo_test]
fn resolve_with_rust_version() {
    Package::new("only-newer", "1.6.0")
        .rust_version("1.65.0")
        .file("src/lib.rs", "fn other_stuff() {}")
        .publish();
    Package::new("newer-and-older", "1.5.0")
        .rust_version("1.55.0")
        .file("src/lib.rs", "fn other_stuff() {}")
        .publish();
    Package::new("newer-and-older", "1.6.0")
        .rust_version("1.65.0")
        .file("src/lib.rs", "fn other_stuff() {}")
        .publish();

    let p = project()
        .file(
            "Cargo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.0.1"
            edition = "2015"
            authors = []
            rust-version = "1.60.0"

            [dependencies]
            only-newer = "1.0.0"
            newer-and-older = "1.0.0"
        "#,
        )
        .file("src/main.rs", "fn main(){}")
        .build();

    p.cargo("generate-lockfile --ignore-rust-version")
        .env(
            "CARGO_RESOLVER_SOMETHING_LIKE_PRECEDENCE",
            "something-like-rust-version",
        )
        .arg("-Zmsrv-policy")
        .masquerade_as_nightly_cargo(&["msrv-policy"])
        .with_stderr(
            "\
[UPDATING] `dummy-registry` index
[LOCKING] 3 packages to latest compatible versions
",
        )
        .run();
    p.cargo("tree")
        .with_stdout(
            "\
foo v0.0.1 ([CWD])
├── newer-and-older v1.6.0
└── only-newer v1.6.0
",
        )
        .run();

    p.cargo("generate-lockfile")
        .env(
            "CARGO_RESOLVER_SOMETHING_LIKE_PRECEDENCE",
            "something-like-rust-version",
        )
        .arg("-Zmsrv-policy")
        .masquerade_as_nightly_cargo(&["msrv-policy"])
        .with_stderr(
            "\
[UPDATING] `dummy-registry` index
[LOCKING] 3 packages to latest Rust 1.60.0 compatible versions
[ADDING] newer-and-older v1.5.0 (latest: v1.6.0)
",
        )
        .run();
    p.cargo("tree")
        .with_stdout(
            "\
foo v0.0.1 ([CWD])
├── newer-and-older v1.5.0
└── only-newer v1.6.0
",
        )
        .run();
}

#[cargo_test]
fn resolve_with_rustc() {
    Package::new("only-newer", "1.6.0")
        .rust_version("1.2345")
        .file("src/lib.rs", "fn other_stuff() {}")
        .publish();
    Package::new("newer-and-older", "1.5.0")
        .rust_version("1.55.0")
        .file("src/lib.rs", "fn other_stuff() {}")
        .publish();
    Package::new("newer-and-older", "1.6.0")
        .rust_version("1.2345")
        .file("src/lib.rs", "fn other_stuff() {}")
        .publish();

    let p = project()
        .file(
            "Cargo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.0.1"
            edition = "2015"
            authors = []
            rust-version = "1.60.0"

            [dependencies]
            only-newer = "1.0.0"
            newer-and-older = "1.0.0"
        "#,
        )
        .file("src/main.rs", "fn main(){}")
        .build();

    p.cargo("generate-lockfile --ignore-rust-version")
        .env(
            "CARGO_RESOLVER_SOMETHING_LIKE_PRECEDENCE",
            "something-like-rust-version",
        )
        .arg("-Zmsrv-policy")
        .masquerade_as_nightly_cargo(&["msrv-policy"])
        .with_stderr(
            "\
[UPDATING] `dummy-registry` index
[LOCKING] 3 packages to latest compatible versions
",
        )
        .run();
    p.cargo("tree")
        .with_stdout(
            "\
foo v0.0.1 ([CWD])
├── newer-and-older v1.6.0
└── only-newer v1.6.0
",
        )
        .run();

    p.cargo("generate-lockfile")
        .env(
            "CARGO_RESOLVER_SOMETHING_LIKE_PRECEDENCE",
            "something-like-rust-version",
        )
        .arg("-Zmsrv-policy")
        .masquerade_as_nightly_cargo(&["msrv-policy"])
        .with_stderr(
            "\
[UPDATING] `dummy-registry` index
[LOCKING] 3 packages to latest Rust [..] compatible versions
[ADDING] newer-and-older v1.5.0 (latest: v1.6.0)
",
        )
        .run();
    p.cargo("tree")
        .with_stdout(
            "\
foo v0.0.1 ([CWD])
├── newer-and-older v1.5.0
└── only-newer v1.6.0
",
        )
        .run();
}

#[cargo_test]
fn resolve_with_backtracking() {
    Package::new("has-rust-version", "1.6.0")
        .rust_version("1.65.0")
        .file("src/lib.rs", "fn other_stuff() {}")
        .publish();
    Package::new("no-rust-version", "2.1.0")
        .file("src/lib.rs", "fn other_stuff() {}")
        .publish();
    Package::new("no-rust-version", "2.2.0")
        .file("src/lib.rs", "fn other_stuff() {}")
        .dep("has-rust-version", "1.6.0")
        .publish();

    let p = project()
        .file(
            "Cargo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.0.1"
            edition = "2015"
            authors = []
            rust-version = "1.60.0"

            [dependencies]
            no-rust-version = "2"
        "#,
        )
        .file("src/main.rs", "fn main(){}")
        .build();

    p.cargo("generate-lockfile --ignore-rust-version")
        .env(
            "CARGO_RESOLVER_SOMETHING_LIKE_PRECEDENCE",
            "something-like-rust-version",
        )
        .arg("-Zmsrv-policy")
        .masquerade_as_nightly_cargo(&["msrv-policy"])
        .with_stderr(
            "\
[UPDATING] `dummy-registry` index
[LOCKING] 3 packages to latest compatible versions
",
        )
        .run();
    p.cargo("tree")
        .with_stdout(
            "\
foo v0.0.1 ([CWD])
└── no-rust-version v2.2.0
    └── has-rust-version v1.6.0
",
        )
        .run();

    // Ideally we'd pick `has-rust-version` 1.6.0 which requires backtracking
    p.cargo("generate-lockfile")
        .env(
            "CARGO_RESOLVER_SOMETHING_LIKE_PRECEDENCE",
            "something-like-rust-version",
        )
        .arg("-Zmsrv-policy")
        .masquerade_as_nightly_cargo(&["msrv-policy"])
        .with_stderr(
            "\
[UPDATING] `dummy-registry` index
[LOCKING] 3 packages to latest Rust 1.60.0 compatible versions
",
        )
        .run();
    p.cargo("tree")
        .with_stdout(
            "\
foo v0.0.1 ([CWD])
└── no-rust-version v2.2.0
    └── has-rust-version v1.6.0
",
        )
        .run();
}

#[cargo_test]
fn resolve_with_multiple_rust_versions() {
    Package::new("only-newer", "1.6.0")
        .rust_version("1.65.0")
        .file("src/lib.rs", "fn other_stuff() {}")
        .publish();
    Package::new("newer-and-older", "1.5.0")
        .rust_version("1.45.0")
        .file("src/lib.rs", "fn other_stuff() {}")
        .publish();
    Package::new("newer-and-older", "1.5.1")
        .rust_version("1.55.0")
        .file("src/lib.rs", "fn other_stuff() {}")
        .publish();
    Package::new("newer-and-older", "1.6.0")
        .rust_version("1.65.0")
        .file("src/lib.rs", "fn other_stuff() {}")
        .publish();

    let p = project()
        .file(
            "Cargo.toml",
            r#"
            [workspace]
            members = ["lower"]

            [package]
            name = "higher"
            version = "0.0.1"
            edition = "2015"
            authors = []
            rust-version = "1.60.0"

            [dependencies]
            only-newer = "1.0.0"
            newer-and-older = "1.0.0"
        "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file(
            "lower/Cargo.toml",
            r#"
            [package]
            name = "lower"
            version = "0.0.1"
            edition = "2015"
            authors = []
            rust-version = "1.50.0"

            [dependencies]
            only-newer = "1.0.0"
            newer-and-older = "1.0.0"
        "#,
        )
        .file("lower/src/main.rs", "fn main() {}")
        .build();

    p.cargo("generate-lockfile --ignore-rust-version")
        .env(
            "CARGO_RESOLVER_SOMETHING_LIKE_PRECEDENCE",
            "something-like-rust-version",
        )
        .arg("-Zmsrv-policy")
        .masquerade_as_nightly_cargo(&["msrv-policy"])
        .with_stderr(
            "\
[UPDATING] `dummy-registry` index
[LOCKING] 4 packages to latest compatible versions
",
        )
        .run();
    p.cargo("tree")
        .with_stdout(
            "\
higher v0.0.1 ([CWD])
├── newer-and-older v1.6.0
└── only-newer v1.6.0
",
        )
        .run();

    p.cargo("generate-lockfile")
        .env(
            "CARGO_RESOLVER_SOMETHING_LIKE_PRECEDENCE",
            "something-like-rust-version",
        )
        .arg("-Zmsrv-policy")
        .masquerade_as_nightly_cargo(&["msrv-policy"])
        .with_stderr(
            "\
[UPDATING] `dummy-registry` index
[LOCKING] 4 packages to latest Rust 1.50.0 compatible versions
[ADDING] newer-and-older v1.5.0 (latest: v1.6.0)
",
        )
        .run();
    p.cargo("tree")
        .with_stdout(
            "\
higher v0.0.1 ([CWD])
├── newer-and-older v1.5.0
└── only-newer v1.6.0
",
        )
        .run();
}

#[cargo_test]
fn resolve_unstable_config_on_stable() {
    Package::new("only-newer", "1.6.0")
        .rust_version("1.65.0")
        .file("src/lib.rs", "fn other_stuff() {}")
        .publish();
    Package::new("newer-and-older", "1.5.0")
        .rust_version("1.55.0")
        .file("src/lib.rs", "fn other_stuff() {}")
        .publish();
    Package::new("newer-and-older", "1.6.0")
        .rust_version("1.65.0")
        .file("src/lib.rs", "fn other_stuff() {}")
        .publish();

    let p = project()
        .file(
            "Cargo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.0.1"
            edition = "2015"
            authors = []
            rust-version = "1.60.0"

            [dependencies]
            only-newer = "1.0.0"
            newer-and-older = "1.0.0"
        "#,
        )
        .file("src/main.rs", "fn main(){}")
        .build();

    p.cargo("generate-lockfile")
        .env(
            "CARGO_RESOLVER_SOMETHING_LIKE_PRECEDENCE",
            "something-like-rust-version",
        )
        .with_stderr(
            "\
[WARNING] ignoring `resolver` config table without `-Zmsrv-policy`
[UPDATING] `dummy-registry` index
[LOCKING] 3 packages to latest compatible versions
",
        )
        .run();
    p.cargo("tree")
        .with_stdout(
            "\
foo v0.0.1 ([CWD])
├── newer-and-older v1.6.0
└── only-newer v1.6.0
",
        )
        .run();

    p.cargo("generate-lockfile")
        .env("CARGO_RESOLVER_SOMETHING_LIKE_PRECEDENCE", "non-existent")
        .with_stderr(
            "\
[WARNING] ignoring `resolver` config table without `-Zmsrv-policy`
[UPDATING] `dummy-registry` index
[LOCKING] 3 packages to latest compatible versions
",
        )
        .run();
    p.cargo("tree")
        .with_stdout(
            "\
foo v0.0.1 ([CWD])
├── newer-and-older v1.6.0
└── only-newer v1.6.0
",
        )
        .run();
}

#[cargo_test(nightly, reason = "edition2024 in rustc is unstable")]
fn resolve_edition2024() {
    Package::new("only-newer", "1.6.0")
        .rust_version("1.65.0")
        .file("src/lib.rs", "fn other_stuff() {}")
        .publish();
    Package::new("newer-and-older", "1.5.0")
        .rust_version("1.55.0")
        .file("src/lib.rs", "fn other_stuff() {}")
        .publish();
    Package::new("newer-and-older", "1.6.0")
        .rust_version("1.65.0")
        .file("src/lib.rs", "fn other_stuff() {}")
        .publish();

    let p = project()
        .file(
            "Cargo.toml",
            r#"
            cargo-features = ["edition2024"]

            [package]
            name = "foo"
            version = "0.0.1"
            edition = "2024"
            authors = []
            rust-version = "1.60.0"

            [dependencies]
            only-newer = "1.0.0"
            newer-and-older = "1.0.0"
        "#,
        )
        .file("src/main.rs", "fn main(){}")
        .build();

    // Edition2024 should resolve for MSRV
    p.cargo("generate-lockfile")
        .arg("-Zmsrv-policy")
        .masquerade_as_nightly_cargo(&["edition2024", "msrv-policy"])
        .with_stderr(
            "\
[UPDATING] `dummy-registry` index
[LOCKING] 3 packages to latest Rust 1.60.0 compatible versions
[ADDING] newer-and-older v1.5.0 (latest: v1.6.0)
",
        )
        .run();
    p.cargo("tree")
        .arg("-Zmsrv-policy")
        .masquerade_as_nightly_cargo(&["edition2024", "msrv-policy"])
        .with_stdout(
            "\
foo v0.0.1 ([CWD])
├── newer-and-older v1.5.0
└── only-newer v1.6.0
",
        )
        .run();

    // `--ignore-rust-version` has precedence over Edition2024
    p.cargo("generate-lockfile --ignore-rust-version")
        .with_stderr(
            "\
[UPDATING] `dummy-registry` index
[LOCKING] 3 packages to latest compatible versions
",
        )
        .arg("-Zmsrv-policy")
        .masquerade_as_nightly_cargo(&["msrv-policy"])
        .run();
    p.cargo("tree")
        .arg("-Zmsrv-policy")
        .masquerade_as_nightly_cargo(&["edition2024", "msrv-policy"])
        .with_stdout(
            "\
foo v0.0.1 ([CWD])
├── newer-and-older v1.6.0
└── only-newer v1.6.0
",
        )
        .run();

    // config has precedence over Edition2024
    p.cargo("generate-lockfile")
        .env(
            "CARGO_RESOLVER_SOMETHING_LIKE_PRECEDENCE",
            "something-like-maximum",
        )
        .with_stderr(
            "\
[UPDATING] `dummy-registry` index
[LOCKING] 3 packages to latest compatible versions
",
        )
        .arg("-Zmsrv-policy")
        .masquerade_as_nightly_cargo(&["msrv-policy"])
        .run();
    p.cargo("tree")
        .arg("-Zmsrv-policy")
        .masquerade_as_nightly_cargo(&["edition2024", "msrv-policy"])
        .with_stdout(
            "\
foo v0.0.1 ([CWD])
├── newer-and-older v1.6.0
└── only-newer v1.6.0
",
        )
        .run();
}

#[cargo_test(nightly, reason = "edition2024 in rustc is unstable")]
fn resolve_v3() {
    Package::new("only-newer", "1.6.0")
        .rust_version("1.65.0")
        .file("src/lib.rs", "fn other_stuff() {}")
        .publish();
    Package::new("newer-and-older", "1.5.0")
        .rust_version("1.55.0")
        .file("src/lib.rs", "fn other_stuff() {}")
        .publish();
    Package::new("newer-and-older", "1.6.0")
        .rust_version("1.65.0")
        .file("src/lib.rs", "fn other_stuff() {}")
        .publish();

    let p = project()
        .file(
            "Cargo.toml",
            r#"
            cargo-features = ["edition2024"]

            [package]
            name = "foo"
            version = "0.0.1"
            edition = "2015"
            authors = []
            rust-version = "1.60.0"
            resolver = "3"

            [dependencies]
            only-newer = "1.0.0"
            newer-and-older = "1.0.0"
        "#,
        )
        .file("src/main.rs", "fn main(){}")
        .build();

    // v3 should resolve for MSRV
    p.cargo("generate-lockfile")
        .arg("-Zmsrv-policy")
        .masquerade_as_nightly_cargo(&["edition2024", "msrv-policy"])
        .with_stderr(
            "\
[UPDATING] `dummy-registry` index
[LOCKING] 3 packages to latest Rust 1.60.0 compatible versions
[ADDING] newer-and-older v1.5.0 (latest: v1.6.0)
",
        )
        .run();
    p.cargo("tree")
        .arg("-Zmsrv-policy")
        .masquerade_as_nightly_cargo(&["edition2024", "msrv-policy"])
        .with_stdout(
            "\
foo v0.0.1 ([CWD])
├── newer-and-older v1.5.0
└── only-newer v1.6.0
",
        )
        .run();

    // `--ignore-rust-version` has precedence over v3
    p.cargo("generate-lockfile --ignore-rust-version")
        .with_stderr(
            "\
[UPDATING] `dummy-registry` index
[LOCKING] 3 packages to latest compatible versions
",
        )
        .arg("-Zmsrv-policy")
        .masquerade_as_nightly_cargo(&["msrv-policy"])
        .run();
    p.cargo("tree")
        .arg("-Zmsrv-policy")
        .masquerade_as_nightly_cargo(&["edition2024", "msrv-policy"])
        .with_stdout(
            "\
foo v0.0.1 ([CWD])
├── newer-and-older v1.6.0
└── only-newer v1.6.0
",
        )
        .run();

    // config has precedence over v3
    p.cargo("generate-lockfile")
        .env(
            "CARGO_RESOLVER_SOMETHING_LIKE_PRECEDENCE",
            "something-like-maximum",
        )
        .with_stderr(
            "\
[UPDATING] `dummy-registry` index
[LOCKING] 3 packages to latest compatible versions
",
        )
        .arg("-Zmsrv-policy")
        .masquerade_as_nightly_cargo(&["msrv-policy"])
        .run();
    p.cargo("tree")
        .arg("-Zmsrv-policy")
        .masquerade_as_nightly_cargo(&["edition2024", "msrv-policy"])
        .with_stdout(
            "\
foo v0.0.1 ([CWD])
├── newer-and-older v1.6.0
└── only-newer v1.6.0
",
        )
        .run();

    // unstable
    p.cargo("generate-lockfile")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] failed to parse manifest at `[CWD]/Cargo.toml`

Caused by:
  the cargo feature `edition2024` requires a nightly version of Cargo, but this is the `stable` channel
  See https://doc.rust-lang.org/book/appendix-07-nightly-rust.html for more information about Rust release channels.
  See https://doc.rust-lang.org/cargo/reference/unstable.html#edition-2024 for more information about using this feature.
",
        )
        .run();
}

#[cargo_test]
fn generate_lockfile_ignore_rust_version_is_unstable() {
    Package::new("bar", "1.5.0")
        .rust_version("1.55.0")
        .file("src/lib.rs", "fn other_stuff() {}")
        .publish();
    Package::new("bar", "1.6.0")
        .rust_version("1.65.0")
        .file("src/lib.rs", "fn other_stuff() {}")
        .publish();

    let p = project()
        .file(
            "Cargo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.0.1"
            edition = "2015"
            authors = []
            rust-version = "1.60.0"
            [dependencies]
            bar = "1.0.0"
        "#,
        )
        .file("src/main.rs", "fn main(){}")
        .build();

    p.cargo("generate-lockfile --ignore-rust-version")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] the `--ignore-rust-version` flag is unstable, and only available on the nightly channel of Cargo, but this is the `stable` channel
See https://doc.rust-lang.org/book/appendix-07-nightly-rust.html for more information about Rust release channels.
See https://github.com/rust-lang/cargo/issues/9930 for more information about the `--ignore-rust-version` flag.
",
        )
        .run();
}

#[cargo_test]
fn update_msrv_resolve() {
    Package::new("bar", "1.5.0")
        .rust_version("1.55.0")
        .file("src/lib.rs", "fn other_stuff() {}")
        .publish();
    Package::new("bar", "1.6.0")
        .rust_version("1.65.0")
        .file("src/lib.rs", "fn other_stuff() {}")
        .publish();

    let p = project()
        .file(
            "Cargo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.0.1"
            edition = "2015"
            authors = []
            rust-version = "1.60.0"
            [dependencies]
            bar = "1.0.0"
        "#,
        )
        .file("src/main.rs", "fn main(){}")
        .build();

    p.cargo("update")
        .env(
            "CARGO_RESOLVER_SOMETHING_LIKE_PRECEDENCE",
            "something-like-rust-version",
        )
        .arg("-Zmsrv-policy")
        .masquerade_as_nightly_cargo(&["msrv-policy"])
        .with_stderr(
            "\
[UPDATING] `dummy-registry` index
[LOCKING] 2 packages to latest Rust 1.60.0 compatible versions
[ADDING] bar v1.5.0 (latest: v1.6.0)
",
        )
        .run();
    p.cargo("update --ignore-rust-version")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] the `--ignore-rust-version` flag is unstable, and only available on the nightly channel of Cargo, but this is the `stable` channel
See https://doc.rust-lang.org/book/appendix-07-nightly-rust.html for more information about Rust release channels.
See https://github.com/rust-lang/cargo/issues/9930 for more information about the `--ignore-rust-version` flag.
",
        )
        .run();
    p.cargo("update --ignore-rust-version")
        .env(
            "CARGO_RESOLVER_SOMETHING_LIKE_PRECEDENCE",
            "something-like-rust-version",
        )
        .arg("-Zmsrv-policy")
        .masquerade_as_nightly_cargo(&["msrv-policy"])
        .with_stderr(
            "\
[UPDATING] `dummy-registry` index
[LOCKING] 1 package to latest compatible version
[UPDATING] bar v1.5.0 -> v1.6.0
",
        )
        .run();
}

#[cargo_test]
fn update_precise_overrides_msrv_resolver() {
    Package::new("bar", "1.5.0")
        .rust_version("1.55.0")
        .file("src/lib.rs", "fn other_stuff() {}")
        .publish();
    Package::new("bar", "1.6.0")
        .rust_version("1.65.0")
        .file("src/lib.rs", "fn other_stuff() {}")
        .publish();

    let p = project()
        .file(
            "Cargo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.0.1"
            edition = "2015"
            authors = []
            rust-version = "1.60.0"
            [dependencies]
            bar = "1.0.0"
        "#,
        )
        .file("src/main.rs", "fn main(){}")
        .build();

    p.cargo("update")
        .env(
            "CARGO_RESOLVER_SOMETHING_LIKE_PRECEDENCE",
            "something-like-rust-version",
        )
        .arg("-Zmsrv-policy")
        .masquerade_as_nightly_cargo(&["msrv-policy"])
        .with_stderr(
            "\
[UPDATING] `dummy-registry` index
[LOCKING] 2 packages to latest Rust 1.60.0 compatible versions
[ADDING] bar v1.5.0 (latest: v1.6.0)
",
        )
        .run();
    p.cargo("update --precise 1.6.0 bar")
        .env(
            "CARGO_RESOLVER_SOMETHING_LIKE_PRECEDENCE",
            "something-like-rust-version",
        )
        .arg("-Zmsrv-policy")
        .masquerade_as_nightly_cargo(&["msrv-policy"])
        .with_stderr(
            "\
[UPDATING] `dummy-registry` index
[UPDATING] bar v1.5.0 -> v1.6.0
",
        )
        .run();
}

#[cargo_test]
fn check_msrv_resolve() {
    Package::new("only-newer", "1.6.0")
        .rust_version("1.65.0")
        .file("src/lib.rs", "fn other_stuff() {}")
        .publish();
    Package::new("newer-and-older", "1.5.0")
        .rust_version("1.55.0")
        .file("src/lib.rs", "fn other_stuff() {}")
        .publish();
    Package::new("newer-and-older", "1.6.0")
        .rust_version("1.65.0")
        .file("src/lib.rs", "fn other_stuff() {}")
        .publish();

    let p = project()
        .file(
            "Cargo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.0.1"
            edition = "2015"
            authors = []
            rust-version = "1.60.0"

            [dependencies]
            only-newer = "1.0.0"
            newer-and-older = "1.0.0"
        "#,
        )
        .file("src/main.rs", "fn main(){}")
        .build();

    p.cargo("check --ignore-rust-version")
        .env(
            "CARGO_RESOLVER_SOMETHING_LIKE_PRECEDENCE",
            "something-like-rust-version",
        )
        .arg("-Zmsrv-policy")
        .masquerade_as_nightly_cargo(&["msrv-policy"])
        .with_stderr(
            "\
[UPDATING] `dummy-registry` index
[LOCKING] 3 packages to latest compatible versions
[DOWNLOADING] crates ...
[DOWNLOADED] [..]
[DOWNLOADED] [..]
[CHECKING] [..]
[CHECKING] [..]
[CHECKING] foo [..]
[FINISHED] `dev` profile [unoptimized + debuginfo] target(s) in [..]s
",
        )
        .run();
    p.cargo("tree")
        .with_stdout(
            "\
foo v0.0.1 ([CWD])
├── newer-and-older v1.6.0
└── only-newer v1.6.0
",
        )
        .run();

    std::fs::remove_file(p.root().join("Cargo.lock")).unwrap();
    p.cargo("check")
        .env(
            "CARGO_RESOLVER_SOMETHING_LIKE_PRECEDENCE",
            "something-like-rust-version",
        )
        .arg("-Zmsrv-policy")
        .masquerade_as_nightly_cargo(&["msrv-policy"])
        .with_stderr(
            "\
[UPDATING] `dummy-registry` index
[LOCKING] 3 packages to latest Rust 1.60.0 compatible versions
[ADDING] newer-and-older v1.5.0 (latest: v1.6.0)
[DOWNLOADING] crates ...
[DOWNLOADED] [..]
[CHECKING] [..]
[CHECKING] foo [..]
[FINISHED] `dev` profile [unoptimized + debuginfo] target(s) in [..]s
",
        )
        .run();
    p.cargo("tree")
        .with_stdout(
            "\
foo v0.0.1 ([CWD])
├── newer-and-older v1.5.0
└── only-newer v1.6.0
",
        )
        .run();
}

#[cargo_test]
fn cargo_install_ignores_msrv_config() {
    Package::new("dep", "1.0.0")
        .rust_version("1.50")
        .file("src/lib.rs", "fn hello() {}")
        .publish();
    Package::new("dep", "1.1.0")
        .rust_version("1.70")
        .file("src/lib.rs", "fn hello() {}")
        .publish();
    Package::new("foo", "0.0.1")
        .rust_version("1.60")
        .file("src/main.rs", "fn main() {}")
        .dep("dep", "1")
        .publish();

    cargo_process("install foo")
        .env(
            "CARGO_RESOLVER_SOMETHING_LIKE_PRECEDENCE",
            "something-like-rust-version",
        )
        .arg("-Zmsrv-policy")
        .masquerade_as_nightly_cargo(&["msrv-policy"])
        .with_stderr(
            "\
[UPDATING] `[..]` index
[DOWNLOADING] crates ...
[DOWNLOADED] foo v0.0.1 (registry [..])
[INSTALLING] foo v0.0.1
[LOCKING] 2 packages to latest compatible versions
[DOWNLOADING] crates ...
[DOWNLOADED] dep v1.1.0 (registry [..])
[COMPILING] dep v1.1.0
[COMPILING] foo v0.0.1
[FINISHED] `release` profile [optimized] target(s) in [..]
[INSTALLING] [CWD]/home/.cargo/bin/foo[EXE]
[INSTALLED] package `foo v0.0.1` (executable `foo[EXE]`)
[WARNING] be sure to add `[..]` to your PATH to be able to run the installed binaries
",
        )
        .run();
}
