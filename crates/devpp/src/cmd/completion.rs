use clap::CommandFactory;
use clap_complete::Shell;
use clap_complete::generate;

use crate::error::Result;

#[derive(clap::Args)]
pub struct Args {
    pub shell: Shell,
}

pub fn run<C: CommandFactory>(args: Args) -> Result<()> {
    generate(args.shell, &mut C::command(), "devpp", &mut std::io::stdout());
    Ok(())
}
