use crate::constants;

use super::{
    jsons::{
        common::McVersionType,
        manifest::{self, McVersionsList},
    },
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

        Ok(VanillaVersion {
            name: format!("Vanilla {}", version.id),
            version: version.id.clone(),
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
    let resp = reqwest::get(constants::VANILLA_VERSIONS).await?;

    Ok(resp.json::<McVersionsList>().await?)
}

/// The Minecraft version configuration
#[derive(Debug)]
pub struct VanillaVersion {
    name: String,
    version: String,
    snapshot: bool,
    /// The URL to the version json
    json_url: String,
}

impl<'a> MinecraftVersion for VanillaVersion {
    fn libraries_json(&self) -> &'static str {
        "https://launchermeta.mojang.com/mc/game/version_manifest.json"
    }

    fn client(&self) -> &'static str {
        "https://launcher.mojang.com/v1/objects/%s/%s"
    }

    fn assets_index(&self) -> &'static str {
        "https://launchermeta.mojang.com/mc/assets/%s/%s"
    }

    fn json_version(&self) -> &'static str {
        ""
    }

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
