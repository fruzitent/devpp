use clap::CommandFactory;

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
        #[arg(short, long, value_hint = clap::ValueHint::FilePath)]
        config: Option<std::path::PathBuf>,
        // @see: https://containers.dev/implementors/spec/#project-workspace-folder
        /// Project workspace folder (typically the root of the git repository)
        #[arg(default_value = ".", value_hint = clap::ValueHint::DirPath)]
        workspace: std::path::PathBuf,
    },
    /// Generate tab-completion scripts for your shell
    Completion { shell: clap_complete::Shell },
}

pub fn generate_shell_completion<G>(generator: G)
where
    G: clap_complete::Generator,
{
    clap_complete::generate(
        generator,
        &mut Args::command(),
        env!("CARGO_BIN_NAME"),
        &mut std::io::stdout(),
    )
}
