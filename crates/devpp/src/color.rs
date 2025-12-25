use anstyle::AnsiColor;
use anstyle::Effects;
use anstyle::Style;
use clap::builder::Styles;

pub const ERROR: Style = AnsiColor::Red.on_default().effects(Effects::BOLD);
pub const HEADER: Style = AnsiColor::Green.on_default().effects(Effects::BOLD);
pub const INVALID: Style = AnsiColor::Yellow.on_default().effects(Effects::BOLD);
pub const LITERAL: Style = AnsiColor::Cyan.on_default().effects(Effects::BOLD);
pub const PLACEHOLDER: Style = AnsiColor::Cyan.on_default();
pub const USAGE: Style = AnsiColor::Green.on_default().effects(Effects::BOLD);
pub const VALID: Style = AnsiColor::Cyan.on_default().effects(Effects::BOLD);

pub const CARGO: Styles = Styles::styled()
    .error(ERROR)
    .header(HEADER)
    .invalid(INVALID)
    .literal(LITERAL)
    .placeholder(PLACEHOLDER)
    .usage(USAGE)
    .valid(VALID);
