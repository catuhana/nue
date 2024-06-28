use std::str::FromStr;

use semver::Version;

#[derive(Clone, Debug)]
pub enum NodeVersion {
    Semver(Version),
    Latest,
    Lts,
}

impl FromStr for NodeVersion {
    type Err = anyhow::Error;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match str {
            "latest" => Ok(Self::Latest),
            "lts" => Ok(Self::Lts),
            _ => {
                let version = str
                    .strip_prefix('v')
                    .map_or(str, |stripped_str| stripped_str);

                match Version::parse(version) {
                    Ok(parsed_version) => Ok(Self::Semver(parsed_version)),
                    Err(_error) => {
                        anyhow::bail!("Invalid semver version (`{version}`) specified. To check all available Node versions, run `list` command.")
                    }
                }
            }
        }
    }
}
