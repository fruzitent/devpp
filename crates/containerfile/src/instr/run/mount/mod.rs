pub mod bind;
pub mod cache;
pub mod secret;
pub mod ssh;
pub mod tmpfs;

use std::fmt::Display;
use std::fmt::Formatter;
use std::path::PathBuf;

use crate::instr::run::mount::bind::BindOptions;
use crate::instr::run::mount::cache::CacheOptions;
use crate::instr::run::mount::secret::SecretOptions;
use crate::instr::run::mount::ssh::SshOptions;
use crate::instr::run::mount::tmpfs::TmpfsOptions;

#[derive(Debug)]
pub enum Mount {
    ///https://github.com/moby/buildkit/blob/dockerfile/1.20.0-labs/frontend/dockerfile/docs/reference.md#run---mounttypebind
    Bind {
        destination: PathBuf,
        options: Option<BindOptions>,
    },
    ///https://github.com/moby/buildkit/blob/dockerfile/1.20.0-labs/frontend/dockerfile/docs/reference.md#run---mounttypecache
    Cache {
        destination: PathBuf,
        options: Option<CacheOptions>,
    },
    ///https://github.com/moby/buildkit/blob/dockerfile/1.20.0-labs/frontend/dockerfile/docs/reference.md#run---mounttypesecret
    Secret {
        options: Option<SecretOptions>,
    },
    Ssh {
        options: Option<SshOptions>,
    },
    ///https://github.com/moby/buildkit/blob/dockerfile/1.20.0-labs/frontend/dockerfile/docs/reference.md#run---mounttypetmpfs
    Tmpfs {
        destination: PathBuf,
        options: Option<TmpfsOptions>,
    },
}

impl Display for Mount {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let mut args = vec![];

        match self {
            Mount::Bind { destination, options } => {
                args.push(String::from("type=bind"));
                args.push(format!("destination={}", destination.to_str().expect("UTF-8")));
                if let Some(options) = options {
                    args.push(format!("{options}"));
                }
            }
            Mount::Cache { destination, options } => {
                args.push(String::from("type=cache"));
                args.push(format!("destination={}", destination.to_str().expect("UTF-8")));
                if let Some(options) = options {
                    args.push(format!("{options}"));
                }
            }
            Mount::Secret { options } => {
                args.push(String::from("type=secret"));
                if let Some(options) = options {
                    args.push(format!("{options}"));
                }
            }
            Mount::Ssh { options } => {
                args.push(String::from("type=ssh"));
                if let Some(options) = options {
                    args.push(format!("{options}"));
                }
            }
            Mount::Tmpfs { destination, options } => {
                args.push(String::from("type=tmpfs"));
                args.push(format!("destination={}", destination.to_str().expect("UTF-8")));
                if let Some(options) = options {
                    args.push(format!("{options}"));
                }
            }
        }

        write!(f, "--mount={}", args.join(","))?;
        Ok(())
    }
}
