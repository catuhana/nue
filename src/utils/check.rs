pub fn path_contains(s: &str) -> anyhow::Result<bool> {
    Ok(std::env::var("PATH")?.contains(s))
}
