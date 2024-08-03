use std::path::PathBuf;

pub mod version;

pub mod jsons;
pub mod vanilla;

/// Returns the path to the official Minecraft folder.
pub fn minecraft_folder() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        let appdata = std::env::var("APPDATA").expect("APPDATA environment variable not found");
        PathBuf::from(appdata).join(".minecraft")
    }

    #[cfg(target_os = "macos")]
    {
        let home = std::env::var("HOME").expect("HOME environment variable not found");
        PathBuf::from(home).join("Library/Application Support/minecraft")
    }

    #[cfg(target_os = "linux")]
    {
        let home = std::env::var("HOME").expect("HOME environment variable not found");
        PathBuf::from(home).join(".minecraft")
    }
}
