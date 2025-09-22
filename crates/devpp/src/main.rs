mod args;
mod devcontainer;
mod feature;

fn main() {
    let args = args::Args::new();
    match args.command {
        args::Commands::Build { config, workspace } => {
            let config = devcontainer::find_config(&workspace, config.as_ref()).unwrap();
            let mut s = std::fs::read_to_string(&config.path).unwrap();
            let devcontainer = devcontainer::DevContainer::from_str(&mut s).unwrap();
            dbg!(&devcontainer);
        }
        args::Commands::Completion { shell } => args::generate_shell_completion(shell),
    }
}
