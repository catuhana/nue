use clap::Args;

use super::NueCommand;

#[derive(Args, Debug)]
pub struct CommandArguments;

impl NueCommand for CommandArguments {
    async fn run(&self) -> anyhow::Result<()> {
        print!("Append these next lines to your shell profile (~/.zshenv, ~/.bashrc, etc.) to make Node and it's tools usable from the command line.\n\n{}", include_str!("../../resources/env.sh"));

        Ok(())
    }
}
