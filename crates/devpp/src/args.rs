#[derive(clap::Parser)]
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
pub enum CommandKind {}
