use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct RawFile {
    pub sha1: String,
    pub size: usize,
    pub url: String,
}
