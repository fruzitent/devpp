pub mod device;
pub mod mount;
pub mod network;
pub mod security;

use std::fmt::Display;
use std::fmt::Formatter;

use crate::instr::run::device::Device;
use crate::instr::run::mount::Mount;
use crate::instr::run::network::Network;
use crate::instr::run::security::Security;

#[derive(Debug, Default)]
pub struct RunOptions {
    ///https://github.com/moby/buildkit/blob/dockerfile/1.20.0-labs/frontend/dockerfile/docs/reference.md#run---device
    pub device: Option<Vec<Device>>,
    ///https://github.com/moby/buildkit/blob/dockerfile/1.20.0-labs/frontend/dockerfile/docs/reference.md#run---mount
    pub mount: Option<Vec<Mount>>,
    ///https://github.com/moby/buildkit/blob/dockerfile/1.20.0-labs/frontend/dockerfile/docs/reference.md#run---network
    pub network: Option<Network>,
    ///https://github.com/moby/buildkit/blob/dockerfile/1.20.0-labs/frontend/dockerfile/docs/reference.md#run---security
    pub security: Option<Security>,
}

impl Display for RunOptions {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let Self {
            device,
            mount,
            network,
            security,
        } = self;
        let mut args = vec![];

        if let Some(device) = device {
            for device in device {
                args.push(format!("{device}"));
            }
        }
        if let Some(mount) = mount {
            for mount in mount {
                args.push(format!("{mount}"));
            }
        }
        if let Some(network) = network {
            args.push(format!("{network}"));
        }
        if let Some(security) = security {
            args.push(format!("{security}"));
        }

        write!(f, "{}", args.join(" "))?;
        Ok(())
    }
}
