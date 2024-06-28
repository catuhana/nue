use clap::Args;

use super::NueCommand;
use crate::types::node_version::NodeVersion;

#[derive(Args, Debug)]
pub struct CommandArguments {
    /// Optional version of Node to install.
    pub version: Option<NodeVersion>,
}

impl NueCommand for CommandArguments {
    type Arguments = Self;

    fn run(&self) -> anyhow::Result<()> {
        println!("Installing Node version: {:?}", self.version);

        Ok(())
    }
}
