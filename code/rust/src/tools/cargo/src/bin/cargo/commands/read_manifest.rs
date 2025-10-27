use crate::command_prelude::*;

pub fn cli() -> Command {
    subcommand("read-manifest")
        .about(color_print::cstr!(
            "\
Print a JSON representation of a Cargo.toml manifest.

Deprecated, use `<cyan,bold>cargo metadata --no-deps</>` instead.\
"
        ))
        .arg_silent_suggestion()
        .arg_manifest_path()
}

pub fn exec(gctx: &mut GlobalContext, args: &ArgMatches) -> CliResult {
    let ws = args.workspace(gctx)?;
    gctx.shell().print_json(&ws.current()?.serialized())?;
    Ok(())
}
