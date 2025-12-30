use std::fmt::Display;
use std::fmt::Formatter;

///https://github.com/moby/buildkit/blob/dockerfile/1.20.0-labs/frontend/dockerfile/docs/reference.md#parser-directives
#[derive(Debug)]
pub enum Directive {
    ///https://github.com/moby/buildkit/blob/dockerfile/1.20.0-labs/frontend/dockerfile/docs/reference.md#check
    Check(String),
    ///https://github.com/moby/buildkit/blob/dockerfile/1.20.0-labs/frontend/dockerfile/docs/reference.md#escape
    Escape(char),
    ///https://github.com/moby/buildkit/blob/dockerfile/1.20.0-labs/frontend/dockerfile/docs/reference.md#syntax
    Syntax(String),
}

impl Display for Directive {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Directive::Check(_) => unimplemented!(),
            Directive::Escape(c) => write!(f, "# escape={c}")?,
            Directive::Syntax(_) => unimplemented!(),
        }
        Ok(())
    }
}
