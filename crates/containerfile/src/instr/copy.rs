use std::fmt::Display;
use std::fmt::Formatter;

use crate::instr::from::FromKind;

#[derive(Debug, Default)]
pub struct CopyOptions {
    ///https://github.com/moby/buildkit/blob/dockerfile/1.20.0-labs/frontend/dockerfile/docs/reference.md#copy---chown---chmod
    pub chmod: Option<String>,
    ///https://github.com/moby/buildkit/blob/dockerfile/1.20.0-labs/frontend/dockerfile/docs/reference.md#copy---chown---chmod
    pub chown: Option<String>,
    ///https://github.com/moby/buildkit/blob/dockerfile/1.20.0-labs/frontend/dockerfile/docs/reference.md#copy---exclude
    pub exclude: Option<String>,
    ///https://github.com/moby/buildkit/blob/dockerfile/1.20.0-labs/frontend/dockerfile/docs/reference.md#copy---from
    pub from: Option<FromKind>,
    ///https://github.com/moby/buildkit/blob/dockerfile/1.20.0-labs/frontend/dockerfile/docs/reference.md#copy---link
    pub link: bool,
    ///https://github.com/moby/buildkit/blob/dockerfile/1.20.0-labs/frontend/dockerfile/docs/reference.md#copy---parents
    pub parents: Option<String>,
}

impl Display for CopyOptions {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let Self {
            chmod,
            chown,
            exclude,
            from,
            link,
            parents,
        } = self;
        let mut args = vec![];

        if let Some(_chmod) = chmod {
            unimplemented!();
        }
        if let Some(_chown) = chown {
            unimplemented!();
        }
        if let Some(_exclude) = exclude {
            unimplemented!();
        }
        if let Some(from) = from {
            let mut s = String::from("--from=");
            match from {
                FromKind::Context(context) => s.push_str(context),
                FromKind::Image {
                    digest,
                    image,
                    repo,
                    tag,
                } => {
                    if let Some(repo) = repo {
                        s.push_str(&format!("{repo}/"));
                    }
                    s.push_str(image);
                    if let Some(tag) = tag {
                        s.push_str(&format!(":{tag}"));
                    }
                    if let Some(digest) = digest {
                        s.push_str(&format!("@{digest}"));
                    }
                }
                FromKind::Stage(stage) => s.push_str(stage),
            }
            args.push(s);
        }
        if *link {
            args.push(String::from("--link"));
        }
        if let Some(_parents) = parents {
            unimplemented!();
        }

        write!(f, "{}", args.join(" "))?;
        Ok(())
    }
}
