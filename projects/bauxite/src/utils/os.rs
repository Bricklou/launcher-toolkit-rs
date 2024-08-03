use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
pub enum OsName {
    #[serde(rename = "osx")]
    MacOs,
    #[serde(rename = "linux")]
    Linux,
    #[serde(rename = "windows")]
    Windows,
    #[serde(untagged)]
    Unknown,
}

impl OsName {
    pub fn current() -> Self {
        #[cfg(target_os = "windows")]
        {
            OsName::Windows
        }
        #[cfg(target_os = "linux")]
        {
            OsName::Linux
        }
        #[cfg(target_os = "macos")]
        {
            OsName::MacOs
        }
        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
        {
            OsName::Unknown
        }
    }
}

impl AsRef<str> for OsName {
    fn as_ref(&self) -> &str {
        match self {
            OsName::MacOs => "osx",
            OsName::Linux => "linux",
            OsName::Windows => "windows",
            OsName::Unknown => "unknown",
        }
    }
}

#[derive(Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
pub struct OperatingSystem {
    name: OsName,
    arch: String,
}

impl OperatingSystem {
    pub fn current() -> Self {
        OperatingSystem {
            name: OsName::current(),
            arch: std::env::consts::ARCH.to_string(),
        }
    }

    pub fn name(&self) -> OsName {
        self.name.clone()
    }

    pub fn name_str(&self) -> &str {
        match self.name {
            OsName::MacOs => "osx",
            OsName::Linux => "linux",
            OsName::Windows => "windows",
            OsName::Unknown => "unknown",
        }
    }

    pub fn arch(&self) -> &str {
        &self.arch
    }
}
