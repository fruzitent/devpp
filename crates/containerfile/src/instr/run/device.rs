use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct Device {
    pub name: String,
    pub options: Option<DeviceOptions>,
}

impl Display for Device {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let Self { name, options } = self;
        let mut args = vec![];

        args.push(name.clone());
        if let Some(options) = options {
            args.push(format!("{options}"));
        }

        write!(f, "{}", args.join(","))?;
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct DeviceOptions {
    pub required: bool,
}

impl Display for DeviceOptions {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let Self { required } = self;
        let mut args = vec![];

        if *required {
            args.push(String::from("required"));
        }

        write!(f, "--device={}", args.join(","))?;
        Ok(())
    }
}
