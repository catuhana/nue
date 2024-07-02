use clap::Parser;
use cli::NueCommand;

mod cli;
mod exts;
mod types;

fn main() -> anyhow::Result<()> {
    match cli::Cli::parse().subcommand {
        cli::Subcommands::Install(install) => install.run(),
        cli::Subcommands::List(list) => list.run(),
    }?;

    Ok(())
}
