pub mod args;
pub mod cmd;
pub mod color;
pub mod error;

use crate::args::Args;
use crate::args::CommandKind;
use crate::error::Result;

fn main() -> Result<()> {
    let args = Args::default();
    match args.command {
        CommandKind::Build(args) => cmd::build::run(args),
        CommandKind::Completion(args) => cmd::completion::run::<Args>(args),
    }
}
