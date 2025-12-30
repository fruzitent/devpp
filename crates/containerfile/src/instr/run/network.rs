use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug, Default)]
pub enum Network {
    #[default]
    Default,
    Host,
    None,
}

impl Display for Network {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "--network=")?;
        match self {
            Network::Default => { /* write!(f, "default")?, */ }
            Network::Host => write!(f, "host")?,
            Network::None => write!(f, "none")?,
        }
        Ok(())
    }
}
