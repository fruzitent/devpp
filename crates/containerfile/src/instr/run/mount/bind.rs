use std::fmt::Display;
use std::fmt::Formatter;
use std::path::PathBuf;

use crate::instr::from::FromKind;

#[derive(Debug, Default)]
pub struct BindOptions {
    pub from: Option<FromKind>,
    pub readwrite: bool,
    pub source: Option<PathBuf>,
}

impl Display for BindOptions {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let Self {
            from,
            readwrite,
            source,
        } = self;
        let mut args = vec![];

        if let Some(from) = from {
            args.push(format!("from={from}"));
        }
        if *readwrite {
            args.push(String::from("readwrite"));
        }
        if let Some(source) = source {
            args.push(format!("source={}", source.to_str().expect("UTF-8")));
        }

        write!(f, "{}", args.join(","))?;
        Ok(())
    }
}
