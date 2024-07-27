use clap::Args;

use super::NueCommand;

#[derive(Args, Debug)]
pub struct CommandArguments;

impl NueCommand for CommandArguments {
    type Arguments = Self;

    async fn run(&self) -> anyhow::Result<()> {
        print!("{}", include_str!("../../resources/env.sh"));

        Ok(())
    }
}
