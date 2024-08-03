use std::{collections::HashMap, path::Path};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AssetsList {
    pub objects: HashMap<String, AssetObject>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AssetObject {
    pub hash: String,
    pub size: u64,
}

#[derive(Debug, thiserror::Error)]
pub enum AssetsListError {
    #[error("There was an error while reading the assets list file")]
    IOError(#[from] std::io::Error),

    #[error("There was an error while parsing the assets list file")]
    ParseError(#[from] serde_json::Error),
}

impl AssetsList {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, AssetsListError> {
        let file = std::fs::File::open(path).map_err(AssetsListError::IOError)?;
        let assets_list = serde_json::from_reader(file).map_err(AssetsListError::ParseError)?;

        Ok(assets_list)
    }
}
