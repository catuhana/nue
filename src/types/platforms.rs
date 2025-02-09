macro_rules! impl_arch_and_traits {
    ($type:ident, $($(#[$($attr:tt)*])? $variant:ident => ($std_arch:expr, $node_arch:expr)),+ $(,)?) => {
        #[derive(Debug)]
        pub enum $type {
            $($(#[$($attr)*])? $variant,)+
        }

        impl $type {
            pub fn current() -> Option<Self> {
                match std::env::consts::ARCH {
                    $($(#[$($attr)*])? $std_arch => Some(Self::$variant),)+
                    _ => None,
                }
            }

            pub const fn node_arch(&self) -> &'static str {
                match self {
                    $($(#[$($attr)*])? Self::$variant => $node_arch,)+
                }
            }
        }
    };
}

#[cfg(target_os = "linux")]
impl_arch_and_traits!(LinuxArch,
    #[cfg(target_arch = "aarch64")]
    ARM64 => ("aarch64", "arm64"),
    #[cfg(target_arch = "arm")]
    ARMv7l => ("arm", "armv7l"),
    #[cfg(target_arch = "powerpc64")]
    Ppc64le => ("powerpc64", "ppc64le"),
    #[cfg(target_arch = "s390x")]
    S390x => ("s390x", "s390x"),
    #[cfg(target_arch = "x86")]
    X64 => ("x86_64", "x64")
);

#[cfg(target_os = "macos")]
impl_arch_and_traits!(MacArch,
    #[cfg(target_arch = "aarch64")]
    ARM64 => ("aarch64", "arm64"),
    #[cfg(target_arch = "x86_64")]
    X64 => ("x86_64", "x64")
);

#[cfg(target_os = "windows")]
impl_arch_and_traits!(WindowsArch,
    #[cfg(target_arch = "aarch64")]
    ARM64 => ("aarch64", "arm64"),
    #[cfg(target_arch = "x86")]
    X86 => ("x86", "x86"),
    #[cfg(target_arch = "x86_64")]
    X64 => ("x86_64", "x64")
);

#[derive(Debug)]
pub enum Platform {
    #[cfg(target_os = "linux")]
    Linux(LinuxArch),
    #[cfg(target_os = "macos")]
    Mac(MacArch),
    #[cfg(target_os = "windows")]
    Windows(WindowsArch),
}

impl Platform {
    pub fn current() -> Option<Self> {
        match std::env::consts::OS {
            #[cfg(target_os = "linux")]
            "linux" => Some(Self::Linux(LinuxArch::current()?)),
            #[cfg(target_os = "macos")]
            "macos" => Some(Self::Mac(MacArch::current()?)),
            #[cfg(target_os = "windows")]
            "windows" => Some(Self::Windows(WindowsArch::current()?)),
            _ => None,
        }
    }

    pub const fn node_archive_extension(&self) -> &'static str {
        match self {
            #[cfg(target_os = "linux")]
            Self::Linux(_) => "tar.xz",
            #[cfg(target_os = "macos")]
            Self::Mac(_) => "tar.xz",
            #[cfg(target_os = "windows")]
            Self::Windows(_) => "7z",
        }
    }

    pub fn node_platform_string(&self) -> String {
        match self {
            #[cfg(target_os = "linux")]
            Self::Linux(arch) => format!("linux-{}", arch.node_arch()),
            #[cfg(target_os = "macos")]
            Self::Mac(arch) => format!("darwin-{}", arch.node_arch()),
            #[cfg(target_os = "windows")]
            Self::Windows(arch) => format!("win-{}", arch.node_arch()),
        }
    }

    pub fn node_index_platform_string(&self) -> String {
        match self {
            #[cfg(target_os = "linux")]
            Self::Linux(arch) => format!("linux-{}", arch.node_arch()),
            #[cfg(target_os = "macos")]
            Self::Mac(arch) => format!("osx-{}-tar", arch.node_arch()),
            #[cfg(target_os = "windows")]
            Self::Windows(arch) => format!("win-{}-7z", arch.node_arch()),
        }
    }
}
