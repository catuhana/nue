use clap::{Parser, Subcommand};

mod env;
mod install;
mod list;
mod uninstall;

pub trait NueCommand {
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
    /// Install or update Node.
    #[command(alias = "update")]
    Install(install::CommandArguments),
    /// Uninstall Node.
    Uninstall(uninstall::CommandArguments),
    /// List all available Node versions.
    List(list::CommandArguments),
    /// Generate environment script.
    Env(env::CommandArguments),
}
