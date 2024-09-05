use clap::Parser;

use cli::NueCommand;

mod cli;
mod constants;
mod exts;
mod globals;
mod types;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    match cli::Cli::parse().subcommand {
        cli::Subcommands::Install(install) => install.run().await,
        cli::Subcommands::Uninstall(uninstall) => uninstall.run().await,
        cli::Subcommands::List(list) => list.run().await,
        cli::Subcommands::Env(env) => env.run().await,
    }?;

    Ok(())
}
