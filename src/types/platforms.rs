#[derive(Debug)]
pub enum Platform {
    Linux(Arch),
}

#[derive(Debug)]
#[allow(dead_code)] // Since `get_system_platforms` depends on the current target architecture, we need to suppress this warning.
pub enum Arch {
    ARM64,
    ARMv7l,
    Ppc64le,
    S390x,
    X64,
}

impl Platform {
    pub const fn get_system_platform() -> Self {
        #[cfg(target_arch = "aarch64")]
        {
            Self::Linux(Arch::ARM64)
        }

        #[cfg(target_arch = "arm")]
        {
            Self::Linux(Arch::ARMv7l)
        }

        #[cfg(target_arch = "powerpc64")]
        {
            Self::Linux(Arch::Ppc64le)
        }

        #[cfg(target_arch = "s390x")]
        {
            Self::Linux(Arch::S390x)
        }

        #[cfg(target_arch = "x86_64")]
        {
            Self::Linux(Arch::X64)
        }
    }
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Linux(arch) => write!(f, "linux-{}", arch),
        }
    }
}

// impl FromStr for Platform {
//     type Err = anyhow::Error;

//     fn from_str(str: &str) -> Result<Self, Self::Err> {
//         let parts: Vec<&str> = str.split('-').collect();

//         match parts.as_slice() {
//             ["linux", arch] => {
//                 let arch = Arch::from_str(arch)?;

//                 Ok(Self::Linux(arch))
//             }
//             _ => anyhow::bail!("Invalid platform specified."),
//         }
//     }
// }

impl std::fmt::Display for Arch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ARM64 => write!(f, "arm64"),
            Self::ARMv7l => write!(f, "armv7l"),
            Self::Ppc64le => write!(f, "ppc64le"),
            Self::S390x => write!(f, "s390x"),
            Self::X64 => write!(f, "x64"),
        }
    }
}

// impl FromStr for Arch {
//     type Err = anyhow::Error;

//     fn from_str(str: &str) -> Result<Self, Self::Err> {
//         match str {
//             "arm64" => Ok(Self::ARM64),
//             "armv7l" => Ok(Self::ARMv7l),
//             "ppc64le" => Ok(Self::Ppc64le),
//             "s390x" => Ok(Self::S390x),
//             "x64" => Ok(Self::X64),
//             _ => anyhow::bail!("Invalid architecture specified."),
//         }
//     }
// }
