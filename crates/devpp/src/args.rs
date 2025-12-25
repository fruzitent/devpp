use crate::cmd;
use crate::color::CARGO;

#[derive(clap::Parser)]
#[clap(styles = CARGO)]
#[command(version)]
pub struct Args {
    #[command(subcommand)]
    pub command: CommandKind,
}

impl Default for Args {
    fn default() -> Self {
        clap::Parser::parse()
    }
}

#[derive(clap::Subcommand)]
pub enum CommandKind {
    /// Build a dev container image
    Build(cmd::build::Args),
    /// Generate tab-completion scripts for your shell
    Completion(cmd::completion::Args),
}
