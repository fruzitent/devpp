mod args;
mod devcontainer;
mod feature;

fn main() {
    let args = args::Args::new();
    match args.command {
        args::Commands::Build { config, workspace } => {
            let config_path = devcontainer::find_config(workspace, config).unwrap();
            let mut s = std::fs::read_to_string(config_path.clone()).unwrap();
            let devcontainer = devcontainer::DevContainer::from_str(&mut s).unwrap();
            dbg!(&devcontainer);
        }
    }
}
