use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum NodeVersion {
    Semver(String),
    Latest,
    Lts,
}

impl FromStr for NodeVersion {
    type Err = anyhow::Error;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match str {
            "latest" => Ok(Self::Latest),
            "lts" => Ok(Self::Lts),
            _ => Ok(Self::Semver(
                str.strip_prefix('v')
                    .map_or(str, |stripped_str| stripped_str)
                    .to_string(),
            )),
        }
    }
}
