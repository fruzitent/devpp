use std::fmt::Display;
use std::fmt::Formatter;
use std::path::PathBuf;

use crate::instr::from::FromKind;

#[derive(Debug, Default)]
pub struct CacheOptions {
    pub from: Option<FromKind>,
    pub gid: Option<u64>,
    pub id: Option<String>,
    pub mode: Option<u64>,
    pub readonly: bool,
    pub sharing: Option<Sharing>,
    pub source: Option<PathBuf>,
    pub uid: Option<u64>,
}

impl Display for CacheOptions {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let Self {
            from,
            gid,
            id,
            mode,
            readonly,
            sharing,
            source,
            uid,
        } = self;
        let mut args = vec![];

        if let Some(from) = from {
            args.push(format!("from={from}"));
        }
        if let Some(_gid) = gid {
            unimplemented!();
        }
        if let Some(id) = id {
            args.push(format!("id={id}"));
        }
        if let Some(_mode) = mode {
            unimplemented!();
        }
        if *readonly {
            args.push(String::from("readonly"));
        }
        if let Some(sharing) = sharing {
            args.push(format!("sharing={sharing}"));
        }
        if let Some(source) = source {
            args.push(format!("source={}", source.to_str().expect("UTF-8")));
        }
        if let Some(_uid) = uid {
            unimplemented!();
        }

        write!(f, "{}", args.join(","))?;
        Ok(())
    }
}

#[derive(Debug, Default)]
pub enum Sharing {
    Locked,
    Private,
    #[default]
    Shared,
}

impl Display for Sharing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Sharing::Locked => write!(f, "locked")?,
            Sharing::Private => write!(f, "private")?,
            Sharing::Shared => write!(f, "shared")?,
        }
        Ok(())
    }
}
