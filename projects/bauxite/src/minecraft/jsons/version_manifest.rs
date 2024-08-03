use std::collections::HashMap;

use serde::Deserialize;
use time::OffsetDateTime;

use crate::utils::os::{OperatingSystem, OsName};

use super::common::McVersionType;

/// The Minecraft version root manifest json.
#[derive(Deserialize, Debug, Clone)]
pub struct McVersionManifest {
    /// Minecraft launcher [`Arguments`] field.
    /// This field appear in 1.13 (17w43a) and replace the [`McVersionManifest::minecraft_arguments`] field.
    arguments: Option<Arguments>,
    /// The game assets index json
    #[serde(rename = "assetIndex")]
    pub asset_index: AssetIndexArtifact,
    /// The assets version
    assets: String,
    /// Its value is 1 for all recent versions of the game (1.16.4 and above) or 0 for all others.
    /// This tag tells the launcher whether it should urge the user to be careful since this version
    /// is older and might not support the latest player safety features.
    #[serde(rename = "complianceLevel", default)]
    compliance_level: u8,
    /// The Minecraft version downloads json.
    pub downloads: Downloads,
    /// The Minecraft version ID.
    pub id: String,
    /// The version of the Java Runtime Environment
    #[serde(rename = "javaVersion", default)]
    java_version: JavaVersion,

    /// The Minecraft version libraries json.
    pub libraries: Vec<Library>,

    /// Logging information for Log4j configuration
    logging: Option<Logging>,

    /// The main game class; for modern versions, it is `net.minecraft.client.main.Main`, but it may differ from older or ancient versions.
    #[serde(rename = "mainClass")]
    main_class: String,

    /// The minimum Launcher version that can run this version of the game.
    #[serde(rename = "minimumLauncherVersion")]
    minimum_launcher_version: u32,

    /// The release date and time.
    #[serde(rename = "releaseTime", with = "time::serde::rfc3339")]
    release_time: OffsetDateTime,

    /// Same as `releaseTime`
    #[serde(with = "time::serde::rfc3339")]
    time: OffsetDateTime,

    /// The type of this game version. It is shown in the version list when you create new installation. The default values are
    /// [`McVersionType::Release`] (`release`) and [`McVersionType::Snapshot`] (`snapshot`).
    #[serde(rename = "type")]
    version_type: McVersionType,

    // LEGACY FIELDS
    /// The Minecraft launcher `minecraftArguments` field.
    /// This field is deprecated in 1.13 (17w43a) and replaced by the [`McVersionManifest::arguments`] field.
    #[serde(rename = "minecraftArguments")]
    minecraft_arguments: Option<String>,
}

/// The Minecraft launcher arguments json.
#[derive(Deserialize, Debug, Clone)]
pub struct Arguments {
    /// The game arguments.
    game: Vec<Argument<GameRule>>,
    /// The jvm arguments.
    jvm: Vec<Argument<OsRule>>,
}

/// The Minecraft argument json.
#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Argument<Rule> {
    /// A simple string argument.
    Simple(String),
    /// A conditional argument.
    /// This argument will be included if the condition is true.
    Conditional {
        /// The condition rules to check
        rules: Vec<Rule>,
        /// The value of the argument
        /// Simple string will be converted into vec<string>, because the value can be a string or a vec<string>
        #[serde(deserialize_with = "crate::utils::serde::deserialize_string_or_seq_string")]
        value: Vec<String>,
    },
}

/// The condition rules to check.
/// This is used in the [`Argument::Conditional`] argument.
#[derive(Deserialize, Debug, Clone)]
pub struct GameRule {
    /// The action to perform.
    action: String,
    /// The feature to check.
    #[serde(default)]
    features: HashMap<String, bool>,
}

/// The Minecraft game asset index json.
#[derive(Deserialize, Debug, Clone)]
pub struct AssetIndexArtifact {
    /// The assets version
    pub id: String,
    /// The SHA1 of the assets file
    pub sha1: String,
    /// The size of the version
    pub size: u64,
    /// The total size of the version
    #[serde(alias = "totalSize")]
    pub total_size: u64,
    /// The URL that the game should visit to download the assets
    pub url: String,
}

/// The Minecraft version downloads json.
#[derive(Deserialize, Debug, Clone)]
pub struct Downloads {
    /// The client.jar download information
    pub client: Artifact,
}

/// The Minecraft artifact json.
#[derive(Deserialize, Debug, Clone)]
pub struct Artifact {
    /// The SHA1 of the artifact
    pub sha1: String,
    /// The size of the artifact
    pub size: u64,
    /// The URL where the artifact is hosted
    pub url: String,
}

/// The Minecraft artifact file json.
/// It is like the [`Artifact`] but with an additional `path` field.
#[derive(Deserialize, Debug, Clone)]
pub struct ArtifactFile {
    /// The path of the artifact
    pub path: String,
    /// The other artifact information
    #[serde(flatten)]
    pub artifact: Artifact,
}

/// The version of the Java Runtime Environment.
#[derive(Deserialize, Debug, Clone)]
pub struct JavaVersion {
    //// Its value for all 1.17 snapshots is "jre-legacy" until 21w18a, and "java-runtime-alpha" since 21w19a.
    component: String,
    /// Its value for all 1.17 snapshots is 8 until 21w18a, 16 until since 1.18-pre1 and 17 since 1.18-pre2.
    #[serde(rename = "majorVersion")]
    major_version: u32,
}

impl Default for JavaVersion {
    fn default() -> Self {
        Self {
            component: "jre-legacy".to_string(),
            major_version: 8,
        }
    }
}

/// A library object
#[derive(Deserialize, Debug, Clone)]
pub struct Library {
    //// The library's download information
    pub downloads: LibraryDownloads,
    /// The maven name for the library, in the form of `group:artifactId:version`
    pub name: String,
    /// The library's URL of the Maven repository (mainly used by Forge)
    pub url: Option<String>,
    /// Information about native libraries (in C) bundled with this library. Appears only when there are classifiers for natives
    #[serde(default)]
    pub natives: HashMap<OsName, String>,
    /// Appears only in two libraries
    pub extract: Option<Extract>,
    /// The extraction rules
    /// Omit if empty
    #[serde(default)]
    pub rules: Vec<OsRule>,
}

/// The os rule
#[derive(Deserialize, Debug, Clone)]
pub struct OsRule {
    /// The action to perform
    pub action: RuleAction,
    /// The OS to check
    #[serde(default)]
    pub os: HashMap<String, String>,
}

/// The rule action
/// This is used in the [`LibraryRule`] struct.
#[derive(Deserialize, Debug, Clone)]
pub enum RuleAction {
    /// The rule action to exclude the library
    #[serde(rename = "allow")]
    Allow,
    /// The rule action to exclude the library
    #[serde(rename = "disallow")]
    Disallow,
}

impl Into<bool> for RuleAction {
    fn into(self) -> bool {
        match self {
            RuleAction::Allow => true,
            RuleAction::Disallow => false,
        }
    }
}

/// The library's download information
#[derive(Deserialize, Debug, Clone)]
pub struct LibraryDownloads {
    /// The artifact download information.
    /// This field is optional may not appear in some libraries.
    pub artifact: Option<ArtifactFile>,
    /// The classifiers download information
    /// This field is optional and only appear in some libraries.
    pub classifiers: Option<HashMap<String, ArtifactFile>>,
}

/// Extract information
#[derive(Deserialize, Debug, Clone)]
pub struct Extract {
    /// Show what to exclude from the extraction
    exclude: Vec<String>,
}

/// Logging information for Log4j configuration
#[derive(Deserialize, Debug, Clone)]
pub struct Logging {
    client: LoggingClient,
}

/// Logging information for Log4j configuration
#[derive(Deserialize, Debug, Clone)]
pub struct LoggingClient {
    /// The JVM argument for adding the log configuration. Its value is "-Dlog4j.configurationFile=${path}"
    argument: String,
    /// The Log4j2 XML configuration used by this version for hte launcher for launcher's log screen.
    file: LoggingArtifact,
    /// Its value is log4j2.xml
    #[serde(rename = "type")]
    log_type: String,
}

/// Logging information for Log4j configuration
#[derive(Deserialize, Debug, Clone)]
pub struct LoggingArtifact {
    /// The artifact ID
    id: String,
    /// The artifact download information
    #[serde(flatten)]
    artifact: Artifact,
}
