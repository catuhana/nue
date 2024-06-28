use clap::Args;

use super::NueCommand;

#[derive(Args, Debug)]
pub struct CommandArguments {
    /// Optional version of Node to install.
    pub version: Option<String>,
}

impl NueCommand for CommandArguments {
    type Arguments = Self;

    fn run(&self) -> anyhow::Result<()> {
        println!("Installing Node version: {:?}", self.version);

        Ok(())
    }
}
