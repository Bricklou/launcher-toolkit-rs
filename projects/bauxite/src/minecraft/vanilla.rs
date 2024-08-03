use tracing::debug;

use crate::constants;

use super::{
    jsons::{common::McVersionType, manifest::McVersionsList, version_manifest::McVersionManifest},
    version::MinecraftVersion,
};

/// A builder for creating a [`VanillaVersion`].
pub struct VanillaVersionBuilder {
    /// The version of Minecraft to build.
    version: VanillaVersionType,
    snapshot: bool,
}

enum VanillaVersionType {
    Latest,
    Version(Box<str>),
}

impl VanillaVersionBuilder {
    /// Create a new builder with the given version.
    pub fn new(version: &str) -> Self {
        debug!("Creating VanillaVersionBuilder with version: {}", version);
        VanillaVersionBuilder {
            version: VanillaVersionType::Version(version.into()),
            snapshot: false,
        }
    }

    /// Create a new builder with the latest version.
    pub fn latest() -> Self {
        VanillaVersionBuilder {
            version: VanillaVersionType::Latest,
            snapshot: false,
        }
    }

    /// Build the [`VanillaVersion`].
    pub async fn build(self) -> Result<VanillaVersion, VanillaVersionError> {
        debug!("Building VanillaVersion");
        // Fetch version json
        let manifest = fetch_version_json().await?;

        let version = match self.version {
            VanillaVersionType::Latest => manifest.latest.release,
            VanillaVersionType::Version(version) => version.to_string(),
        };

        let version = match manifest.versions.iter().find(|v| v.id == version) {
            Some(version) => version,
            None => return Err(VanillaVersionError::VersionNotFound),
        };

        let version_manifest = fetch_version_manifest_json(&version.url).await?;

        Ok(VanillaVersion {
            name: format!("Vanilla {}", version.id),
            id: version.id.clone(),
            version: version_manifest,
            json_url: version.url.clone(),
            snapshot: version.version_type == McVersionType::Snapshot,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum VanillaVersionError {
    #[error("Version not found")]
    VersionNotFound,
    #[error("Failed to fetch version manifest")]
    FetchVersionManifest(#[from] reqwest::Error),
}

pub async fn fetch_version_json() -> Result<McVersionsList, reqwest::Error> {
    debug!("Fetching version manifest list");
    reqwest::get(constants::VANILLA_VERSIONS)
        .await?
        .json()
        .await
}

pub async fn fetch_version_manifest_json(url: &str) -> Result<McVersionManifest, reqwest::Error> {
    debug!("Fetching version manifest json from {}", url);
    reqwest::get(url).await?.json().await
}

/// The Minecraft version configuration
#[derive(Debug)]
pub struct VanillaVersion {
    name: String,
    id: String,
    version: McVersionManifest,
    snapshot: bool,
    /// The URL to the version json
    json_url: String,
}

impl<'a> MinecraftVersion for VanillaVersion {
    fn name(&self) -> &String {
        &self.name
    }

    fn is_snapshot(&self) -> bool {
        self.snapshot
    }

    fn json_url(&self) -> &String {
        &self.json_url
    }
}
