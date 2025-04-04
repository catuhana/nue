mod commands;

// TODO: Use `thiserror` for error handling.
pub trait Command {
    fn run(&self) -> anyhow::Result<()>;
}

#[derive(Debug, clap::Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: Subcommands,
}

#[derive(Debug, clap::Subcommand)]
pub enum Subcommands {
    Clean(commands::clean::Arguments),
    Env(commands::env::Arguments),
    // TODO: Consider changing the behaviour of `use`.
    #[command(alias = "update", alias = "use")]
    Install(commands::install::Arguments),
    List(commands::list::Arguments),
    Uninstall(commands::uninstall::Arguments),
}

impl Cli {
    #[must_use]
    pub fn parse() -> Self {
        <Self as clap::Parser>::parse()
    }
}
