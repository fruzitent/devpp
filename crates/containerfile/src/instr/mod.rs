pub mod copy;
pub mod directive;
pub mod from;
pub mod run;

use std::fmt::Display;
use std::fmt::Formatter;
use std::path::PathBuf;

use crate::instr::copy::CopyOptions;
use crate::instr::directive::Directive;
use crate::instr::from::FromKind;
use crate::instr::run::RunOptions;

///https://github.com/moby/buildkit/blob/dockerfile/1.20.0-labs/frontend/dockerfile/docs/reference.md#format
#[derive(Debug)]
pub enum Instr {
    ///https://github.com/moby/buildkit/blob/dockerfile/1.20.0-labs/frontend/dockerfile/docs/reference.md#add
    Add,
    ///https://github.com/moby/buildkit/blob/dockerfile/1.20.0-labs/frontend/dockerfile/docs/reference.md#arg
    Arg(Vec<(String, Option<String>)>),
    ///https://github.com/moby/buildkit/blob/dockerfile/1.20.0-labs/frontend/dockerfile/docs/reference.md#cmd
    Cmd,
    Comment(String),
    ///https://github.com/moby/buildkit/blob/dockerfile/1.20.0-labs/frontend/dockerfile/docs/reference.md#copy
    Copy {
        ///https://github.com/moby/buildkit/blob/dockerfile/1.20.0-labs/frontend/dockerfile/docs/reference.md#destination-1
        destination: PathBuf,
        options: Option<CopyOptions>,
        ///https://github.com/moby/buildkit/blob/dockerfile/1.20.0-labs/frontend/dockerfile/docs/reference.md#source-1
        source: Vec<PathBuf>,
    },
    Directive(Directive),
    Empty,
    ///https://github.com/moby/buildkit/blob/dockerfile/1.20.0-labs/frontend/dockerfile/docs/reference.md#entrypoint
    Entrypoint,
    ///https://github.com/moby/buildkit/blob/dockerfile/1.20.0-labs/frontend/dockerfile/docs/reference.md#env
    Env(Vec<(String, String)>),
    ///https://github.com/moby/buildkit/blob/dockerfile/1.20.0-labs/frontend/dockerfile/docs/reference.md#expose
    Expose,
    ///https://github.com/moby/buildkit/blob/dockerfile/1.20.0-labs/frontend/dockerfile/docs/reference.md#from
    From {
        kind: FromKind,
        name: Option<String>,
        platform: Option<String>,
    },
    ///https://github.com/moby/buildkit/blob/dockerfile/1.20.0-labs/frontend/dockerfile/docs/reference.md#healthcheck
    Healthcheck,
    ///https://github.com/moby/buildkit/blob/dockerfile/1.20.0-labs/frontend/dockerfile/docs/reference.md#label
    Label,
    ///https://github.com/moby/buildkit/blob/dockerfile/1.20.0-labs/frontend/dockerfile/docs/reference.md#maintainer
    Maintainer,
    ///https://github.com/moby/buildkit/blob/dockerfile/1.20.0-labs/frontend/dockerfile/docs/reference.md#onbuild
    Onbuild,
    ///https://github.com/moby/buildkit/blob/dockerfile/1.20.0-labs/frontend/dockerfile/docs/reference.md#run
    Run {
        command: Vec<String>,
        options: Option<RunOptions>,
    },
    ///https://github.com/moby/buildkit/blob/dockerfile/1.20.0-labs/frontend/dockerfile/docs/reference.md#shell
    Shell,
    ///https://github.com/moby/buildkit/blob/dockerfile/1.20.0-labs/frontend/dockerfile/docs/reference.md#stopsignal
    Stopsignal,
    ///https://github.com/moby/buildkit/blob/dockerfile/1.20.0-labs/frontend/dockerfile/docs/reference.md#user
    User,
    ///https://github.com/moby/buildkit/blob/dockerfile/1.20.0-labs/frontend/dockerfile/docs/reference.md#volume
    Volume,
    ///https://github.com/moby/buildkit/blob/dockerfile/1.20.0-labs/frontend/dockerfile/docs/reference.md#workdir
    Workdir,
}

impl Instr {
    pub fn display(&self, escape: Option<char>) -> InstrDisplay<'_> {
        InstrDisplay {
            escape: escape.unwrap_or(DEFAULT_ESCAPE),
            inner: self,
        }
    }
}

impl Display for Instr {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        self.display(None).fmt(f)
    }
}

#[derive(Debug)]
pub struct InstrDisplay<'a> {
    escape: char,
    inner: &'a Instr,
}

impl<'a> Display for InstrDisplay<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self.inner {
            Instr::Add => {
                unimplemented!();
            }
            Instr::Arg(inner) => {
                assert!(!inner.is_empty());
                write!(f, "ARG")?;
                for (name, default_value) in inner {
                    write!(f, " {name}")?;
                    if let Some(default_value) = default_value {
                        write!(f, "={}", escape_str(self.escape, default_value))?;
                    }
                }
            }
            Instr::Cmd => {
                write!(f, "CMD")?;
                unimplemented!();
            }
            Instr::Comment(inner) => write!(f, "# {inner}")?,
            Instr::Copy {
                destination,
                options,
                source,
            } => {
                assert!(!source.is_empty());
                write!(f, "COPY")?;
                if let Some(options) = options {
                    write!(f, " {options}")?;
                }
                let mut args = vec![];
                for path in source {
                    args.push(escape_str(self.escape, path.to_str().expect("UTF-8")));
                }
                args.push(escape_str(self.escape, destination.to_str().expect("UTF-8")));
                write!(f, " [{}]", args.join(", "))?;
            }
            Instr::Directive(directive) => write!(f, "{directive}")?,
            Instr::Empty => {}
            Instr::Entrypoint => {
                write!(f, "ENTRYPOINT")?;
                unimplemented!();
            }
            Instr::Env(inner) => {
                assert!(!inner.is_empty());
                write!(f, "ENV")?;
                for (key, value) in inner {
                    write!(f, " {key}={}", escape_str(self.escape, value))?;
                }
            }
            Instr::Expose => {
                write!(f, "EXPOSE")?;
                unimplemented!();
            }
            Instr::From { kind, name, platform } => {
                write!(f, "FROM")?;
                if let Some(platform) = platform {
                    write!(f, " --platform={platform}")?;
                }
                write!(f, " {kind}")?;
                if let Some(name) = name {
                    write!(f, " AS {name}")?;
                }
            }
            Instr::Healthcheck => {
                write!(f, "HEALTHCHECK")?;
                unimplemented!();
            }
            Instr::Label => {
                write!(f, "LABEL")?;
                unimplemented!();
            }
            Instr::Maintainer => {
                write!(f, "MAINTAINER")?;
                unimplemented!();
            }
            Instr::Onbuild => {
                write!(f, "ONBUILD")?;
                unimplemented!();
            }
            Instr::Run { command, options } => {
                assert!(!command.is_empty());
                write!(f, "RUN")?;
                if let Some(options) = options {
                    write!(f, " {options}")?;
                }
                let mut args = vec![];
                for arg in command {
                    args.push(escape_str(self.escape, arg));
                }
                write!(f, " [{}]", args.join(", "))?;
            }
            Instr::Shell => {
                write!(f, "SHELL")?;
                unimplemented!();
            }
            Instr::Stopsignal => {
                write!(f, "STOPSIGNAL")?;
                unimplemented!();
            }
            Instr::User => {
                write!(f, "USER")?;
                unimplemented!();
            }
            Instr::Volume => {
                write!(f, "VOLUME")?;
                unimplemented!();
            }
            Instr::Workdir => {
                write!(f, "WORKDIR")?;
                unimplemented!();
            }
        }
        Ok(())
    }
}

pub const DEFAULT_ESCAPE: char = '\\';

fn escape_str(escape: char, s: &str) -> String {
    let double = format!("{escape}{escape}");
    let quote = format!("{escape}\"");
    format!("\"{}\"", s.replace(escape, &double).replace('"', &quote))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::instr::run::mount::Mount;

    #[test]
    fn instr_arg() {
        let instr = Instr::Arg(vec![
            (String::from("FOO"), None),
            (String::from("BAR"), Some(String::from("BAZ"))),
        ]);
        assert_eq!(format!("{instr}"), String::from("ARG FOO BAR=\"BAZ\""));
    }

    #[test]
    fn instr_copy() {
        let instr = Instr::Copy {
            destination: PathBuf::from("/app/baz/"),
            options: Some(CopyOptions {
                link: true,
                ..Default::default()
            }),
            source: vec![PathBuf::from("./foo/"), PathBuf::from("./bar/")],
        };
        assert_eq!(
            format!("{instr}"),
            String::from("COPY --link [ \"./foo/\", \"./bar/\", \"/app/baz/\" ]")
        );
    }

    #[test]
    fn instr_env() {
        let instr = Instr::Env(vec![
            (String::from("FOO"), String::from("BAR")),
            (String::from("BAZ"), String::from("QUIX")),
        ]);
        assert_eq!(format!("{instr}"), String::from("ENV FOO=\"BAR\" BAZ=\"QUIX\""));
    }

    #[test]
    fn instr_from() {
        let instr = Instr::From {
            kind: FromKind::Image {
                digest: Some(String::from("sha256:000")),
                image: String::from("foo/bar"),
                repo: Some(String::from("example.org")),
                tag: Some(String::from("baz")),
            },
            name: Some(String::from("test")),
            platform: Some(String::from("linux/amd64")),
        };
        assert_eq!(
            format!("{instr}"),
            String::from("FROM --platform=linux/amd64 example.org/foo/bar:baz@sha256:000 AS test")
        );
    }

    #[test]
    fn instr_run() {
        let instr = Instr::Run {
            command: vec![String::from("foo"), String::from("--bar=42")],
            options: Some(RunOptions {
                mount: Some(vec![Mount::Tmpfs {
                    destination: PathBuf::from("/tmp/"),
                    options: None,
                }]),
                ..Default::default()
            }),
        };
        assert_eq!(
            format!("{instr}"),
            String::from("RUN --mount=type=tmpfs,destination=/tmp/ [ \"foo\", \"--bar=42\" ]")
        );
    }
}
