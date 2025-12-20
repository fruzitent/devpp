use std::process::exit;

use crate::args::Args;
use crate::args::CommandKind;
use crate::args::generate_shell_completion;

pub mod args;

fn main() {
    tracing_subscriber::fmt::init();

    let args = Args::default();
    match args.command {
        CommandKind::Build { config, workspace } => {
            if let Err(error) = devpp::build(&workspace, config.as_deref()) {
                tracing::error!("{error}");
                exit(1);
            }
        }
        CommandKind::Completion { shell } => generate_shell_completion(shell),
    }
}
