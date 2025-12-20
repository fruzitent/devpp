use std::path::PathBuf;

use anstyle::AnsiColor;
use anstyle::Style;
use clap::CommandFactory;
use clap::builder::styling::Styles;
use clap_complete::Generator;
use clap_complete::Shell;

pub const STYLE_ERROR: Style = AnsiColor::Red.on_default().effects(anstyle::Effects::BOLD);
pub const STYLE_HEADER: Style = AnsiColor::Green.on_default().effects(anstyle::Effects::BOLD);
pub const STYLE_INVALID: Style = AnsiColor::Yellow.on_default().effects(anstyle::Effects::BOLD);
pub const STYLE_LITERAL: Style = AnsiColor::Cyan.on_default().effects(anstyle::Effects::BOLD);
pub const STYLE_PLACEHOLDER: Style = AnsiColor::Cyan.on_default();
pub const STYLE_USAGE: Style = AnsiColor::Green.on_default().effects(anstyle::Effects::BOLD);
pub const STYLE_VALID: Style = AnsiColor::Cyan.on_default().effects(anstyle::Effects::BOLD);

pub const ARGS_STYLING: Styles = Styles::styled()
    .error(STYLE_ERROR)
    .header(STYLE_HEADER)
    .invalid(STYLE_INVALID)
    .literal(STYLE_LITERAL)
    .placeholder(STYLE_PLACEHOLDER)
    .usage(STYLE_USAGE)
    .valid(STYLE_VALID);

#[derive(clap::Parser)]
#[clap(styles = ARGS_STYLING)]
#[command(version)]
pub struct Args {
    #[command(subcommand)]
    pub command: CommandKind,
}

impl Default for Args {
    fn default() -> Self {
        clap::Parser::parse()
    }
}

#[derive(clap::Subcommand)]
pub enum CommandKind {
    /// Build a dev container image
    Build {
        // @see: https://containers.dev/implementors/spec/#devcontainerjson
        /// devcontainer.json path
        #[arg(short, long, value_hint = clap::ValueHint::FilePath)]
        config: Option<PathBuf>,
        // @see: https://containers.dev/implementors/spec/#project-workspace-folder
        /// Project workspace folder (typically the root of the git repository)
        #[arg(default_value = ".", value_hint = clap::ValueHint::DirPath)]
        workspace: PathBuf,
    },
    /// Generate tab-completion scripts for your shell
    Completion { shell: Shell },
}

pub fn generate_shell_completion<G: Generator>(generator: G) {
    clap_complete::generate(generator, &mut Args::command(), "devpp", &mut std::io::stdout())
}
