pub mod args;

fn main() {
    let args = args::Args::new();
    match args.command {
        args::Commands::Build { config, workspace } => {
            if let Err(error) = devpp::build(workspace, config) {
                eprintln!("{style}error:{style:#} {error}", style = args::STYLE_ERROR);
                std::process::exit(1);
            }
        }
        args::Commands::Completion { shell } => args::generate_shell_completion(shell),
    }
}
