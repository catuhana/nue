use anyhow::Context;

use crate::types;

#[derive(Debug)]
pub struct NodeURLs {
    pub home: &'static str,
    pub github: &'static str,
}

impl NodeURLs {
    pub async fn fetch_releases(&self) -> anyhow::Result<Vec<types::node::Release>> {
        let response = reqwest::get(self.get_releases_index()).await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to fetch releases: {}", response.status());
        }

        let releases_json = response
            .json()
            .await
            .context("Failed to parse releases JSON")?;

        Ok(releases_json)
    }

    pub fn get_releases_index(&self) -> String {
        format!("{}/dist/index.json", self.home)
    }

    pub fn get_distribution_path(&self) -> String {
        format!("{}/dist", self.home)
    }
}

impl Default for NodeURLs {
    fn default() -> Self {
        Self {
            home: "https://nodejs.org",
            github: "https://github.com/nodejs/node",
        }
    }
}
