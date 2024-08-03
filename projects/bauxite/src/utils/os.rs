use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
pub enum OperatingSystem {
    #[serde(rename = "osx")]
    MacOs,
    #[serde(rename = "linux")]
    Linux,
    #[serde(rename = "windows")]
    Windows,
    #[serde(untagged)]
    Unknown,
}

impl OperatingSystem {
    pub fn current() -> Self {
        #[cfg(target_os = "windows")]
        {
            OperatingSystem::Windows
        }
        #[cfg(target_os = "linux")]
        {
            OperatingSystem::Linux
        }
        #[cfg(target_os = "macos")]
        {
            OperatingSystem::MacOs
        }
        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
        {
            OperatingSystem::Unknown
        }
    }
}

impl ToString for OperatingSystem {
    fn to_string(&self) -> String {
        match self {
            OperatingSystem::MacOs => "osx".to_string(),
            OperatingSystem::Linux => "linux".to_string(),
            OperatingSystem::Windows => "windows".to_string(),
            OperatingSystem::Unknown => "unknown".to_string(),
        }
    }
}

impl From<String> for OperatingSystem {
    fn from(s: String) -> Self {
        match s.as_str() {
            "osx" => OperatingSystem::MacOs,
            "linux" => OperatingSystem::Linux,
            "windows" => OperatingSystem::Windows,
            _ => OperatingSystem::Unknown,
        }
    }
}
