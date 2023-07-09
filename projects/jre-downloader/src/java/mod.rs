use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

use crate::error::{self, JreResult};

use self::download::download_file::DownloadFile;

pub mod manifest;
pub mod download;

#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum FileType {
    Directory,
    File,
    Link,
}

#[derive(Deserialize, Debug, Clone)]
pub struct JreFileDownload {
    pub raw: DownloadFile,
}

#[derive(Deserialize, Debug, Clone)]
pub struct JreFile {
    #[serde(rename = "type")]
    pub file_type: FileType,
    pub executable: Option<bool>,
    pub downloads: Option<JreFileDownload>,
    pub target: Option<String>,
}

pub async fn get_jre_files(url: &String) -> JreResult<HashMap<String, JreFile>> {
    let response_result = reqwest::get(url).await;
    if response_result.is_err() {
        return Err(error::JreError::ManifestReadError);
    }
    let response = response_result.unwrap();
    if !response.status().is_success() {
        return Err(error::JreError::ManifestReadError);
    }
    let response_text = response.text().await;
    if response_text.is_err() {
        return Err(error::JreError::ManifestReadError);
    }

    let json: Value = serde_json::from_str(&response_text.unwrap()).unwrap();

    let json_files = &json.as_object().unwrap()["files"];

    let mut resp = HashMap::new();
    for (key, value) in json_files.as_object().unwrap() {
        let file:Result<JreFile, serde_json::Error>  = serde_json::from_value(value.clone());
        if file.is_err() {
            println!("Error while parsing file: {}", file.err().unwrap());
            return Err(error::JreError::ManifestParseError);
        }
        resp.insert(key.clone(), file.unwrap());
    }

    Ok(resp)
    //Ok(resp)
}