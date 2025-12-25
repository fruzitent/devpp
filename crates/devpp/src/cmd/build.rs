use std::path::PathBuf;

use crate::error::Result;

#[derive(clap::Args)]
pub struct Args {
    // @see: https://containers.dev/implementors/spec/#devcontainerjson
    /// devcontainer.json path
    #[arg(short, long, value_hint = clap::ValueHint::FilePath)]
    pub config: Option<PathBuf>,
    // @see: https://containers.dev/implementors/spec/#project-workspace-folder
    /// Project workspace folder (typically the root of the git repository)
    #[arg(default_value = ".", value_hint = clap::ValueHint::DirPath)]
    pub workspace: PathBuf,
}

pub fn run(args: Args) -> Result<()> {
    Ok(devpp_core::build(
        &mut std::io::stdout(),
        &args.workspace,
        args.config.as_deref(),
    )?)
}
