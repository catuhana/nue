#[derive(Debug)]
pub enum Platform {
    #[cfg(target_os = "linux")]
    Linux(Arch),
    #[cfg(target_os = "macos")]
    Mac(Arch),
}

// TODO: Create an enum similar to this for `NodeRelease`'s `files` field.
#[derive(Debug)]
pub enum Arch {
    #[cfg(target_arch = "aarch64")]
    ARM64,
    #[cfg(target_arch = "arm")]
    ARMv7l,
    #[cfg(target_arch = "powerpc64")]
    Ppc64le,
    #[cfg(target_arch = "s390x")]
    S390x,
    #[cfg(target_arch = "x86_64")]
    X64,
}

impl Platform {
    pub const fn get_system_platform() -> Self {
        #[cfg(target_arch = "aarch64")]
        {
            #[cfg(target_os = "linux")]
            {
                Self::Linux(Arch::ARM64)
            }
            #[cfg(target_os = "macos")]
            {
                Self::Mac(Arch::ARM64)
            }
        }

        #[cfg(target_arch = "arm")]
        {
            #[cfg(target_os = "linux")]
            {
                Self::Linux(Arch::ARMv7l)
            }
            #[cfg(target_os = "macos")]
            {
                Self::Mac(Arch::ARMv7l)
            }
        }

        #[cfg(target_arch = "powerpc64")]
        {
            #[cfg(target_os = "linux")]
            {
                Self::Linux(Arch::Ppc64le)
            }
            #[cfg(target_os = "macos")]
            {
                Self::Mac(Arch::Ppc64le)
            }
        }

        #[cfg(target_arch = "s390x")]
        {
            #[cfg(target_os = "linux")]
            {
                Self::Linux(Arch::S390x)
            }
            #[cfg(target_os = "macos")]
            {
                Self::Mac(Arch::S390x)
            }
        }

        #[cfg(target_arch = "x86_64")]
        {
            #[cfg(target_os = "linux")]
            {
                Self::Linux(Arch::X64)
            }
            #[cfg(target_os = "macos")]
            {
                Self::Mac(Arch::X64)
            }
        }
    }
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(target_os = "linux")]
            Self::Linux(arch) => write!(f, "linux-{arch}"),
            #[cfg(target_os = "macos")]
            Self::Mac(arch) => write!(f, "darwin-{arch}"),
        }
    }
}

impl std::fmt::Display for Arch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(target_arch = "aarch64")]
            Self::ARM64 => write!(f, "arm64"),
            #[cfg(target_arch = "arm")]
            Self::ARMv7l => write!(f, "armv7l"),
            #[cfg(target_arch = "powerpc64")]
            Self::Ppc64le => write!(f, "ppc64le"),
            #[cfg(target_arch = "s390x")]
            Self::S390x => write!(f, "s390x"),
            #[cfg(target_arch = "x86_64")]
            Self::X64 => write!(f, "x64"),
        }
    }
}
