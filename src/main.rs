use clap::Parser as _;

use cli::NueCommand as _;

mod cli;
mod constants;
mod exts;
mod globals;
mod types;
mod utils;

fn main() -> anyhow::Result<()> {
    match cli::Cli::parse().subcommand {
        cli::Subcommands::Install(install) => install.run(),
        cli::Subcommands::Uninstall(uninstall) => uninstall.run(),
        cli::Subcommands::List(list) => list.run(),
        cli::Subcommands::Env(env) => env.run(),
        cli::Subcommands::Clean(clean) => clean.run(),
    }?;

    Ok(())
}
