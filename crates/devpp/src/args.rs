pub const STYLE_ERROR: anstyle::Style = anstyle::AnsiColor::Red.on_default().effects(anstyle::Effects::BOLD);
pub const STYLE_HEADER: anstyle::Style = anstyle::AnsiColor::Green.on_default().effects(anstyle::Effects::BOLD);
pub const STYLE_INVALID: anstyle::Style = anstyle::AnsiColor::Yellow.on_default().effects(anstyle::Effects::BOLD);
pub const STYLE_LITERAL: anstyle::Style = anstyle::AnsiColor::Cyan.on_default().effects(anstyle::Effects::BOLD);
pub const STYLE_PLACEHOLDER: anstyle::Style = anstyle::AnsiColor::Cyan.on_default();
pub const STYLE_USAGE: anstyle::Style = anstyle::AnsiColor::Green.on_default().effects(anstyle::Effects::BOLD);
pub const STYLE_VALID: anstyle::Style = anstyle::AnsiColor::Cyan.on_default().effects(anstyle::Effects::BOLD);

pub const ARGS_STYLING: clap::builder::styling::Styles = clap::builder::styling::Styles::styled()
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
pub enum CommandKind {}
