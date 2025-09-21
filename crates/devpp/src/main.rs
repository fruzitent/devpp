mod args;
mod devcontainer;
mod feature;

fn main() {
    let args = args::Args::new();
    match args.command {
        args::Commands::Build { config, workspace } => {
            let config_path = devcontainer::find_config(workspace, config).unwrap();
            dbg!(&config_path);
        }
    }
}
