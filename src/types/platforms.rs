use std::fmt;

macro_rules! impl_arch_and_traits {
    ($type:ident, $($variant:ident => ($const_arch:expr, $node_arch:expr)),+ $(,)?) => {
        #[derive(Debug)]
        pub enum $type {
            $($variant,)+
        }

        impl $type {
            pub fn current() -> Option<Self> {
                match std::env::consts::ARCH {
                    $($const_arch => Some(Self::$variant),)+
                    _ => None,
                }
            }
        }

        impl fmt::Display for $type {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $(
                        Self::$variant => write!(f, $node_arch),
                    )+
                }
            }
        }
    };
}

impl_arch_and_traits!(LinuxArch,
    ARM64 => ("arm64", "arm64"),
    ARMv7l => ("arm", "armv7l"),
    Ppc64le => ("powerpc64", "ppc64le"),
    S390x => ("s390x", "s390x"),
    X64 => ("x86_64", "x64")
);

impl_arch_and_traits!(MacArch,
    ARM64 => ("arm64", "arm64"),
    X64 => ("x86_64", "x64")
);

impl_arch_and_traits!(WindowsArch,
    ARM64 => ("arm64", "arm64"),
    X86 => ("x86", "x86"),
    X64 => ("x86_64", "x64")
);

#[derive(Debug)]
pub enum Platform {
    Linux(LinuxArch),
    Mac(MacArch),
    Windows(WindowsArch),
}

impl Platform {
    pub fn current() -> Option<Self> {
        match std::env::consts::OS {
            "linux" => Some(Self::Linux(LinuxArch::current()?)),
            "macos" => Some(Self::Mac(MacArch::current()?)),
            "windows" => Some(Self::Windows(WindowsArch::current()?)),
            _ => None,
        }
    }

    pub const fn archive_extension(&self) -> &'static str {
        match self {
            Self::Linux(_) | Self::Mac(_) => "tar.xz",
            Self::Windows(_) => "zip",
        }
    }

    pub fn platform_string(&self) -> String {
        match self {
            Self::Linux(arch) => format!("linux-{arch}"),
            Self::Mac(arch) => format!("darwin-{arch}"),
            Self::Windows(arch) => format!("win-{arch}"),
        }
    }

    pub fn download_index_platform_string(&self) -> String {
        match self {
            Self::Linux(arch) => format!("linux-{arch}"),
            Self::Mac(arch) => format!("osx-{arch}-tar"),
            Self::Windows(arch) => format!("win-{arch}-7z"),
        }
    }
}
