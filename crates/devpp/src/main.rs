pub mod args;

fn main() {
    tracing_subscriber::fmt::init();

    let args = args::Args::default();
    match args.command {
        args::CommandKind::Build { config, workspace } => {
            if let Err(error) = devpp::build(&workspace, config.as_deref()) {
                tracing::error!("{error}");
                std::process::exit(1);
            }
        }
        args::CommandKind::Completion { shell } => args::generate_shell_completion(shell),
    }
}
