pub mod args;

fn main() {
    tracing_subscriber::fmt::init();

    let _args = args::Args::default();
}
