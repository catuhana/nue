use std::path::Path;

#[cfg(windows)]
const ENVIRONMENT_SCRIPT: &str = include_str!("../resources/env.ps1");
#[cfg(unix)]
const ENVIRONMENT_SCRIPT: &str = include_str!("../resources/env.sh");

pub fn create_env_script(path: impl AsRef<Path>) -> anyhow::Result<()> {
    Ok(std::fs::write(path.as_ref(), ENVIRONMENT_SCRIPT)?)
}
