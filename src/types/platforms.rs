use std::str::FromStr;

#[derive(Debug)]
pub enum Platform {
    Linux(Arch),
}

#[derive(Debug)]
pub enum Arch {
    ARM64,
    ARMv7l,
    Ppc64le,
    S390x,
    X64,
}

impl FromStr for Platform {
    type Err = anyhow::Error;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = str.split('-').collect();

        match parts.as_slice() {
            ["linux", arch] => {
                let arch = Arch::from_str(arch)?;

                Ok(Self::Linux(arch))
            }
            _ => anyhow::bail!("Invalid platform specified."),
        }
    }
}

impl FromStr for Arch {
    type Err = anyhow::Error;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match str {
            "arm64" => Ok(Self::ARM64),
            "armv7l" => Ok(Self::ARMv7l),
            "ppc64le" => Ok(Self::Ppc64le),
            "s390x" => Ok(Self::S390x),
            "x64" => Ok(Self::X64),
            _ => anyhow::bail!("Invalid architecture specified."),
        }
    }
}
