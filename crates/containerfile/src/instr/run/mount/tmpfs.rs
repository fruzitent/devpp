use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug, Default)]
pub struct TmpfsOptions {
    pub size: Option<String>,
}

impl Display for TmpfsOptions {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let Self { size } = self;
        let mut args = vec![];

        if let Some(size) = size {
            args.push(size.clone());
        }

        write!(f, "{}", args.join(","))?;
        Ok(())
    }
}
