use serde::Deserialize;
use time::OffsetDateTime;

use super::common::McVersionType;

#[derive(Deserialize, Debug, Clone)]
pub struct McVersionsList {
    pub latest: Latest,
    pub versions: Vec<Version>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Latest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Version {
    pub id: String,
    #[serde(rename = "type")]
    pub version_type: McVersionType,
    pub url: String,
    #[serde(rename = "time", with = "time::serde::rfc3339")]
    pub time: OffsetDateTime,
    #[serde(rename = "releaseTime", with = "time::serde::rfc3339")]
    pub release_time: OffsetDateTime,
}
