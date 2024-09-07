use std::fmt;

#[derive(Debug)]
pub enum Platform {
    #[cfg(target_os = "linux")]
    Linux(Arch),
    #[cfg(target_os = "macos")]
    Mac(Arch),
}

#[derive(Debug)]
pub enum Arch {
    #[cfg(all(target_arch = "aarch64", any(target_os = "linux", target_os = "macos")))]
    ARM64,
    #[cfg(all(target_arch = "arm", target_os = "linux"))]
    ARMv7l,
    #[cfg(all(target_arch = "powerpc64", target_os = "linux"))]
    Ppc64le,
    #[cfg(all(target_arch = "s390x", target_os = "linux"))]
    S390x,
    #[cfg(all(target_arch = "x86_64", any(target_os = "linux", target_os = "macos")))]
    X64,
}

impl Platform {
    pub const fn get_system_platform() -> Self {
        #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
        {
            Self::Linux(Arch::ARM64)
        }
        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        {
            Self::Mac(Arch::ARM64)
        }
        #[cfg(all(target_os = "linux", target_arch = "arm"))]
        {
            Self::Linux(Arch::ARMv7l)
        }
        #[cfg(all(target_os = "macos", target_arch = "arm"))]
        {
            Self::Mac(Arch::ARMv7l)
        }
        #[cfg(all(target_os = "linux", target_arch = "powerpc64"))]
        {
            Self::Linux(Arch::Ppc64le)
        }
        #[cfg(all(target_os = "macos", target_arch = "powerpc64"))]
        {
            Self::Mac(Arch::Ppc64le)
        }
        #[cfg(all(target_os = "linux", target_arch = "s390x"))]
        {
            Self::Linux(Arch::S390x)
        }
        #[cfg(all(target_os = "macos", target_arch = "s390x"))]
        {
            Self::Mac(Arch::S390x)
        }
        #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
        {
            Self::Linux(Arch::X64)
        }
        #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
        {
            Self::Mac(Arch::X64)
        }
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(target_os = "linux")]
            Self::Linux(arch) => write!(f, "linux-{arch}"),
            #[cfg(target_os = "macos")]
            Self::Mac(arch) => write!(f, "darwin-{arch}"),
        }
    }
}

impl fmt::Display for Arch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
