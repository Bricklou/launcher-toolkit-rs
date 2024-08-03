use std::path::Path;

use tokio::task::JoinSet;
use tracing::debug;

use crate::{
    minecraft::{
        jsons::version_manifest::{ArtifactFile, Library, McVersionManifest, OsRule},
        minecraft_folder,
        version::MinecraftVersion,
    },
    utils::{
        download::{retry_download, DownloadError, DownloadInfo},
        os::OperatingSystem,
    },
};

/// Update the vanilla Minecraft files.
pub async fn update_vanilla(
    version: &dyn MinecraftVersion,
    output: impl AsRef<Path>,
) -> Result<(), VanillaUpdateError> {
    debug!("Updating vanilla Minecraft version: {}", version.name());

    let manifest = version.manifest();

    let minecraft_folder = minecraft_folder();

    if !minecraft_folder.exists() {
        debug!(
            "Minecraft folder does not exist, creating it at {:?}",
            minecraft_folder
        );
        std::fs::create_dir_all(&minecraft_folder).expect("Failed to create Minecraft folder");
    } else {
        debug!("Found minecraft folder at {:?}", minecraft_folder);
    }

    // Download libraries
    download_libraries(manifest, minecraft_folder).await?;

    // Download assets index

    // Download client

    // Download natives

    // Download assets

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum VanillaUpdateError {
    #[error("Failed to download the libraries")]
    DownloadError(#[from] DownloadError),

    #[error("No artifact found for the library")]
    NoArtifact,
}

async fn download_libraries(
    manifest: &McVersionManifest,
    minecraft_folder: impl AsRef<Path>,
) -> Result<(), VanillaUpdateError> {
    debug!("Downloading libraries");

    let libraries_folder = minecraft_folder.as_ref().join("libraries");

    if !libraries_folder.exists() {
        debug!(
            "Libraries folder does not exist, creating it at {:?}",
            libraries_folder
        );
        std::fs::create_dir_all(&libraries_folder).expect("Failed to create libraries folder");
    } else {
        debug!("Found libraries folder at {:?}", libraries_folder);
    }

    let mut joinset = JoinSet::new();

    for library in &manifest.libraries {
        debug!("Downloading library: {}", library.name);

        if library.downloads.artifact.is_none() {
            continue;
        }

        let library = library.clone();
        let libraries_folder = libraries_folder.clone();
        joinset.spawn(async move {
            // Download library
            download_library(library, libraries_folder).await
        });
    }

    while let Some(res) = joinset.join_next().await {
        let res = res.unwrap();
        debug!("Downloaded library: {:?}", res);
    }

    Ok(())
}

async fn download_library(
    library: Library,
    libraries_folder: impl AsRef<Path>,
) -> Result<String, VanillaUpdateError> {
    debug!("Downloading library: {}", library.name);

    // Get the artifact file information
    let file = match library.downloads.artifact {
        Some(file) => file,
        None => return Err(VanillaUpdateError::NoArtifact),
    };

    let lib_path = libraries_folder.as_ref().join(file.path);

    // Download the library
    retry_download(DownloadInfo {
        path: lib_path,
        url: file.artifact.url,
        sha1: file.artifact.sha1,
        size: file.artifact.size,
    })
    .await?;

    Ok(library.name)
}
