use nue::Command as _;

fn main() -> anyhow::Result<()> {
    match nue::Cli::parse().subcommand {
        nue::Subcommands::Clean(arguments) => arguments.run(),
        nue::Subcommands::Env(arguments) => arguments.run(),
        nue::Subcommands::Install(arguments) => arguments.run(),
        nue::Subcommands::List(arguments) => arguments.run(),
        nue::Subcommands::Uninstall(arguments) => arguments.run(),
    }
}
