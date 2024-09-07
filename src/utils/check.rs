use std::env;

pub fn path_contains(s: &str) -> anyhow::Result<bool> {
    Ok(env::var("PATH")?.contains(s))
}
