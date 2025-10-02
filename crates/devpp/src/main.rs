pub mod args;

fn main() {
    tracing_subscriber::fmt::init();

    let args = args::Args::default();
    match args.command {
        args::CommandKind::Completion { shell } => args::generate_shell_completion(shell),
    }
}
