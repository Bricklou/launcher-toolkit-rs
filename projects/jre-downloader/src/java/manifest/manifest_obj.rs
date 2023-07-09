use crate::{constants, error::JreResult, java::download::download_file::DownloadFile};
use std::collections::HashMap;

use serde::Deserialize;
use time::OffsetDateTime;

#[derive(Deserialize, Debug, Clone)]
pub struct RuntimeVersion {
    pub name: String,
    /// Release time
    #[serde(with = "time::serde::rfc3339")]
    pub released: OffsetDateTime,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RuntimeData {
    pub manifest: DownloadFile,
    pub version: RuntimeVersion,
}

#[derive(Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum RequestType {
    #[serde(rename = "java-runtime-alpha")]
    JavaRuntimeAlpha,
    #[serde(rename = "java-runtime-beta")]
    JavaRuntimeBeta,
    #[serde(rename = "java-runtime-gamma")]
    JavaRuntimeGamma,
    #[serde(rename = "jre-legacy")]
    JreLegacy,
    #[serde(rename = "minecraft-java-exe")]
    MinecraftJavaExe,
}

#[derive(Deserialize, Debug, Clone)]
pub struct JreManifest {
    pub linux: HashMap<RequestType, Vec<RuntimeData>>,
    #[serde(rename = "linux-i386")]
    pub linux_i386: HashMap<RequestType, Vec<RuntimeData>>,
    #[serde(rename = "mac-os")]
    pub mac_os: HashMap<RequestType, Vec<RuntimeData>>,
    #[serde(rename = "mac-os-arm64")]
    pub mac_os_arm64: HashMap<RequestType, Vec<RuntimeData>>,
    #[serde(rename = "windows-x64")]
    pub windows: HashMap<RequestType, Vec<RuntimeData>>,
    #[serde(rename = "windows-x86")]
    pub windows_32: HashMap<RequestType, Vec<RuntimeData>>,
    #[serde(rename = "windows-arm64")]
    pub windows_arm: HashMap<RequestType, Vec<RuntimeData>>,
}

impl JreManifest {
    /// Get the JRE manifest from Mojang servers.
    pub async fn get() -> JreResult<Self> {
        let response = reqwest::Client::new()
            .get(constants::JRE_MANIFEST_URL)
            .send()
            .await?
            .error_for_status()?
            .json::<JreManifest>()
            .await?;

        Ok(response)
    }
}


#[cfg(test)]
mod tests {
    use crate::java::manifest::JreManifest;

    #[tokio::test]
    async fn test_manifest_alpha(){
    let url = JreManifest::get().await;
    assert!(url.is_ok());
    }
}