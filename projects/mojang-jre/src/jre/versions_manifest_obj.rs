use std::collections::HashMap;

use serde::Deserialize;
use time::OffsetDateTime;
use tracing::debug;

use crate::{
    constants,
    errors::{JreError, JreResult},
    jre::manifest_files::JreManifestFilesList,
};

use super::{file::RawFile, manifest_files::JreFile};

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
pub enum VersionType {
    #[serde(rename = "java-runtime-alpha")]
    JavaRuntimeAlpha,
    #[serde(rename = "java-runtime-beta")]
    JavaRuntimeBeta,
    #[serde(rename = "java-runtime-gamma")]
    JavaRuntimeGamma,
    #[serde(rename = "java-runtime-gamma-snapshot")]
    JavaRuntimeGammaSnapshot,
    #[serde(rename = "jre-legacy")]
    Legacy,
    #[serde(rename = "minecraft-java-exe")]
    MinecraftJavaExe,
    /// This is a catch-all for any other version types that may be added in the future
    #[serde(untagged)]
    Unknown(String),
}

#[derive(Deserialize, Debug, Clone)]
pub struct RuntimeVersion {
    pub name: String,
    /// Release time
    #[serde(with = "time::serde::rfc3339")]
    pub released: OffsetDateTime,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RuntimeData {
    pub manifest: RawFile,
    pub version: RuntimeVersion,
}

#[derive(Deserialize, Debug, Clone)]
pub struct VersionsManifest {
    pub linux: HashMap<VersionType, Vec<RuntimeData>>,
}

impl VersionsManifest {
    /// Get the JRE manifest from Mojang servers.
    pub async fn get() -> JreResult<Self> {
        debug!("Getting the JRE manifest from Mojang servers");
        let response = reqwest::Client::new()
            .get(constants::JRE_MANIFEST_URL)
            .send()
            .await?
            .error_for_status()?
            .json::<VersionsManifest>()
            .await?;

        Ok(response)
    }

    pub async fn get_files(
        &self,
        version_type: &VersionType,
    ) -> JreResult<HashMap<String, JreFile>> {
        debug!(message = "Getting the files", version_type = ?version_type);
        let url = self.get_runtime_url(version_type)?;

        debug!(message = "Getting the files from the URL", url = ?url);
        let resp = reqwest::get(url).await?;

        if !resp.status().is_success() {
            return Err(JreError::ManifestReadError);
        }

        Ok(resp.json::<JreManifestFilesList>().await?.files)
    }
}

pub trait JreManifestExt {
    fn get_runtime_url(&self, version_type: &VersionType) -> JreResult<String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_manifest_alpha() {
        let url = VersionsManifest::get().await;
        assert!(url.is_ok());
    }
}
