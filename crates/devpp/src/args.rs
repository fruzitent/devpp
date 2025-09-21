#[derive(clap::Parser)]
#[command(version)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

impl Args {
    pub fn new() -> Self {
        clap::Parser::parse()
    }
}

#[derive(clap::Subcommand)]
pub enum Commands {
    /// Build a dev container image
    Build {
        // @see: https://containers.dev/implementors/spec/#devcontainerjson
        /// devcontainer.json path
        #[arg(short, long)]
        config: Option<std::path::PathBuf>,
        // @see: https://containers.dev/implementors/spec/#project-workspace-folder
        /// Project workspace folder (typically the root of the git repository)
        #[arg(default_value = ".")]
        workspace: std::path::PathBuf,
    },
}
