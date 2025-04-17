use core::convert::Infallible;
use std::{
    ffi::OsStr,
    io::Read as _,
    path::{Path, PathBuf},
};

use indicatif::{ProgressBar, ProgressStyle};
use nue_common::{
    constants::NODE_DISTRIBUTIONS_URL,
    globals::{ARGV_0, NUE_CACHE_PATH, NUE_NODE_PATH, NUE_PATH},
};
use nue_types::{
    node::{lts::Lts, release::Release},
    platforms::Platform,
};
use ureq::BodyReader;

use crate::Command;

#[derive(Debug, clap::Args)]
pub struct Arguments {
    /// Node version to install.
    #[arg(default_value_t = VersionInputs::default())]
    pub version: VersionInputs,

    /// Force install even if the version is already installed.
    #[arg(short, long)]
    pub force: bool,

    /// Do not use the cache.
    #[arg(short, long)]
    pub no_cache: bool,
}

#[derive(Clone, Debug, Default)]
pub enum VersionInputs {
    VersionString(String),
    #[default]
    Latest,
    Lts(Option<String>),
}

impl Command for Arguments {
    fn run(&self) -> anyhow::Result<()> {
        let progress = ProgressBar::new_spinner();
        progress.enable_steady_tick(core::time::Duration::from_millis(100));

        progress.set_message("Fetching releases...");
        let releases = Self::fetch_all()?;

        progress.set_message("Filtering releases...");
        let selected_release = match &self.version {
            VersionInputs::VersionString(version) => releases
                .iter()
                .find(|release| release.version.to_string().starts_with(version)),
            VersionInputs::Lts(lts) => releases.iter().find(|release| lts.as_ref().map_or_else(|| release.lts.is_code_name(), |code_name| matches!(&release.lts, Lts::CodeName(name) if &name.to_lowercase() == code_name))),
            VersionInputs::Latest => releases.iter().max_by_key(|release| &release.version),
        };

        match selected_release {
            Some(release) => {
                if let Some(version) = Self::check_installed()? {
                    if version == format!("v{}", release.version) && !self.force {
                        progress.finish_with_message(format!(
                            "Node {version} is already installed. Use `--force` to re-install."
                        ));
                        return Ok(());
                    }
                }

                let platform = Platform::system().expect("unsupported platform");
                if Self::check_cached()?.iter().any(|cached| {
                    cached.file_name() == Some(OsStr::new(&release.get_archive_string(&platform)))
                }) && !self.no_cache
                {
                    release.install_from_cache(&platform, &progress)?;
                } else {
                    release.install(&platform, &progress)?;
                }

                progress.finish_with_message(format!("Node {} is now installed.", release.version));

                if !super::is_node_in_path() {
                    println!(
                        "Node is installed, but its path isn't in `PATH`. Run `{} env` to fix it.",
                        &*ARGV_0
                    );
                }
            }
            None => anyhow::bail!("No release found for version {}", self.version),
        }

        Ok(())
    }
}

impl Arguments {
    fn fetch_all() -> anyhow::Result<Vec<Release>> {
        Ok(ureq::get(format!("{NODE_DISTRIBUTIONS_URL}/index.json"))
            .call()?
            .body_mut()
            .read_json()?)
    }

    fn check_installed<'a>() -> anyhow::Result<Option<String>> {
        if !NUE_NODE_PATH.try_exists()? {
            return Ok(None);
        }

        let stdout = std::process::Command::new(
            #[cfg(unix)]
            {
                NUE_NODE_PATH.join("bin").join("node")
            },
            #[cfg(windows)]
            {
                NUE_NODE_PATH.join("node.exe")
            },
        )
        .arg("--version")
        .output()?
        .stdout;

        Ok(Some(String::from_utf8(stdout.trim_ascii().to_vec())?))
    }

    fn check_cached() -> anyhow::Result<Vec<PathBuf>> {
        let mut caches = vec![];
        match std::fs::read_dir(&*NUE_CACHE_PATH) {
            Ok(entries) => {
                for entry in entries {
                    let entry = entry?;
                    let path = entry.path();

                    if !path.is_dir() {
                        continue;
                    }

                    if entry.file_name().to_string_lossy().starts_with("node-") {
                        caches.push(path);
                    }
                }

                Ok(caches)
            }
            Err(_) => Ok(caches),
        }
    }
}

trait ReleaseExt
where
    Self: Sized,
{
    fn download_node_archive(
        &self,
        platform: &Platform,
    ) -> anyhow::Result<(BodyReader<'static>, Option<usize>)>;
    fn extract_node_archive(&self, platform: &Platform, chunks: &[u8]) -> anyhow::Result<()>;

    fn install(&self, platform: &Platform, progress: &ProgressBar) -> anyhow::Result<()>;
    fn install_from_cache(&self, platform: &Platform, progress: &ProgressBar)
    -> anyhow::Result<()>;

    fn link_from_cache(
        &self,
        platform: &Platform,
        destination: impl AsRef<Path>,
    ) -> anyhow::Result<()>;
}

impl ReleaseExt for Release {
    fn download_node_archive(
        &self,
        platform: &Platform,
    ) -> anyhow::Result<(BodyReader<'static>, Option<usize>)> {
        let response = ureq::get(self.get_download_url(platform)).call()?;
        let content_length = response.headers().get("Content-Length").and_then(|value| {
            value
                .to_str()
                .ok()
                .and_then(|value| value.parse::<usize>().ok())
        });

        Ok((response.into_body().into_reader(), content_length))
    }

    fn extract_node_archive(&self, platform: &Platform, chunks: &[u8]) -> anyhow::Result<()> {
        let cached_file_path = NUE_CACHE_PATH.join(self.get_archive_string(platform));
        if cached_file_path.try_exists()? {
            std::fs::remove_dir_all(&cached_file_path)?;
        }

        #[cfg(unix)]
        {
            use binstall_tar::Archive;
            use liblzma::decode_all;

            let decoded = decode_all(chunks)?;
            Archive::new(decoded.as_slice()).unpack(&*NUE_CACHE_PATH)?;
        }

        #[cfg(windows)]
        {
            use sevenz_rust2::decompress;

            decompress(std::io::Cursor::new(chunks), &*NUE_CACHE_PATH)?;
        }

        Ok(())
    }

    fn install(&self, platform: &Platform, progress: &ProgressBar) -> anyhow::Result<()> {
        self.is_supported_by_current_platform(platform)?;

        let (mut file_reader, file_size) = self.download_node_archive(platform)?;

        let download_progress_bar = match file_size {
            Some(file_size) => {
                progress.set_length(file_size as u64);
                progress.set_style(
                    #[expect(clippy::literal_string_with_formatting_args, reason = "Clippy thinks we're trying to format a string here.")]
                    ProgressStyle::default_bar()
                        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} {bytes_per_sec} ({eta})")?
                        .progress_chars("-Cc·")
                );
                progress
            }
            None => progress,
        };

        progress.set_message(format!("Downloading Node {}...", self.version));
        let mut file = Vec::with_capacity(file_size.unwrap_or(32 * 1024 * 1024));
        let mut buffer = vec![0; 1024 * 1024];
        while let Ok(chunk_len) = file_reader.read(&mut buffer) {
            if chunk_len == 0 {
                break;
            }

            file.extend_from_slice(&buffer[..chunk_len]);
            download_progress_bar.inc(chunk_len as u64);
        }
        progress.set_style(ProgressStyle::default_spinner());

        progress.set_message("Extracting...");
        self.extract_node_archive(platform, &file)?;

        progress.set_message("Linking...");
        self.link_from_cache(platform, &*NUE_NODE_PATH)?;

        Ok(())
    }

    fn install_from_cache(
        &self,
        platform: &Platform,
        progress: &ProgressBar,
    ) -> anyhow::Result<()> {
        if NUE_NODE_PATH.try_exists()? {
            std::fs::remove_dir_all(&*NUE_NODE_PATH)?;
        }

        progress.set_message("Linking...");
        self.link_from_cache(platform, &*NUE_NODE_PATH)
    }

    fn link_from_cache(
        &self,
        platform: &Platform,
        destination: impl AsRef<Path>,
    ) -> anyhow::Result<()> {
        let cache_path = NUE_PATH
            .join("cache")
            .join(self.get_archive_string(platform));

        #[cfg(unix)]
        std::os::unix::fs::symlink(cache_path, destination)?;

        #[cfg(windows)]
        std::os::windows::fs::symlink_dir(cache_path, destination)?;

        Ok(())
    }
}

impl core::fmt::Display for VersionInputs {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::VersionString(version) => write!(f, "{version}"),
            Self::Latest => write!(f, "latest"),
            Self::Lts(Some(code_name)) => write!(f, "{code_name}"),
            Self::Lts(None) => write!(f, "lts"),
        }
    }
}

impl core::str::FromStr for VersionInputs {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match s.as_str() {
            "latest" => Ok(Self::Latest),
            "lts" => Ok(Self::Lts(None)),
            s if s.parse::<node_semver::Range>().is_ok() => {
                if s.starts_with('v') {
                    return Ok(Self::VersionString(s[1..].to_string()));
                }

                Ok(Self::VersionString(s.to_string()))
            }
            s => Ok(Self::Lts(Some(s.to_string()))),
        }
    }
}
