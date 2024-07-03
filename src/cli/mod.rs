use clap::{Args, Parser, Subcommand};

mod install;
mod list;

pub trait NueCommand {
    type Arguments: Args;

    async fn run(&self) -> anyhow::Result<()>;
}

#[derive(Parser, Debug)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: Subcommands,
}

#[derive(Subcommand, Debug)]
pub enum Subcommands {
    /// Install Node.
    Install(install::CommandArguments),
    /// List all available Node versions.
    List(list::CommandArguments),
}
