use crate::Command;

#[derive(Debug, clap::Args)]
pub struct Arguments;

impl Command for Arguments {
    fn run(&self) -> anyhow::Result<()> {
        todo!("Implement `clean` command");
    }
}
