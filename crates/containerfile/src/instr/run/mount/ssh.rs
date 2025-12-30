use std::fmt::Display;
use std::fmt::Formatter;
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct SshOptions {
    pub destination: Option<PathBuf>,
    pub gid: Option<u64>,
    pub id: Option<String>,
    pub mode: Option<u64>,
    pub required: bool,
    pub uid: Option<u64>,
}

impl Display for SshOptions {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let Self {
            destination,
            gid,
            id,
            mode,
            required,
            uid,
        } = self;
        let mut args = vec![];

        if let Some(destination) = destination {
            args.push(format!("destination={}", destination.to_str().expect("UTF-8")));
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
        if *required {
            args.push(String::from("required"));
        }
        if let Some(_uid) = uid {
            unimplemented!();
        }

        write!(f, "{}", args.join(","))?;
        Ok(())
    }
}
