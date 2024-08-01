use std::collections::HashMap;

use serde::Deserialize;

use super::file::RawFile;

#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum FileType {
    Directory,
    File,
    Link,
}

#[derive(Deserialize, Debug, Clone)]
pub struct JreFileDownload {
    pub raw: RawFile,
}

#[derive(Deserialize, Debug, Clone)]
pub struct JreFile {
    #[serde(rename = "type")]
    pub file_type: FileType,
    #[serde(default)]
    pub executable: bool,
    pub downloads: Option<JreFileDownload>,
    pub target: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct JreManifestFilesList {
    pub files: HashMap<String, JreFile>,
}
