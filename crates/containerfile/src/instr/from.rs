use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum FromKind {
    Context(String),
    Image {
        digest: Option<String>,
        image: String,
        repo: Option<String>,
        tag: Option<String>,
    },
    Stage(String),
}

impl Display for FromKind {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            FromKind::Context(context) => write!(f, "{context}")?,
            FromKind::Image {
                digest,
                image,
                repo,
                tag,
            } => {
                if let Some(repo) = repo {
                    write!(f, "{repo}/")?;
                }
                write!(f, "{image}")?;
                if let Some(tag) = tag {
                    write!(f, ":{tag}")?;
                }
                if let Some(digest) = digest {
                    write!(f, "@{digest}")?;
                }
            }
            FromKind::Stage(stage) => write!(f, "{stage}")?,
        }

        Ok(())
    }
}
