use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug, Default)]
pub enum Security {
    Insecure,
    #[default]
    Sandbox,
}

impl Display for Security {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "--security=")?;
        match self {
            Security::Insecure => write!(f, "insecure")?,
            Security::Sandbox => { /* write!(f, "sandbox")?, */ }
        }
        Ok(())
    }
}
