use clap::Args;

use crate::types;

use super::NueCommand;

#[derive(Args, Debug)]
pub struct CommandArguments;

impl NueCommand for CommandArguments {
    type Arguments = Self;

    fn run(&self) -> anyhow::Result<()> {
        let response: Vec<types::node::Release> =
            ureq::get("https://nodejs.org/download/release/index.json")
                .call()?
                .into_json()?;

        let releases = response
            .iter()
            .map(|release| release.version.to_string())
            .collect::<Vec<String>>()
            .join("\n");

        println!("{releases}");

        Ok(())
    }
}
