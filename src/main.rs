use clap::Parser;
use cli::NueCommand;

mod cli;
mod types;

fn main() -> anyhow::Result<()> {
    match cli::Cli::parse().subcommand {
        cli::Subcommands::Install(install) => install.run(),
    }?;

    Ok(())
}
