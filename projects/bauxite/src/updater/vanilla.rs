use std::path::Path;

use tokio::task::JoinSet;
use tracing::debug;

use crate::{
    constants,
    minecraft::{
        jsons::{
            assets_index::{AssetObject, AssetsList, AssetsListError},
            version_manifest::{ArtifactFile, Library, McVersionManifest},
        },
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
    download_libraries(manifest, &minecraft_folder).await?;

    // Download assets index
    download_assets_index(manifest, &minecraft_folder).await?;

    // Download client
    download_client(manifest, &minecraft_folder).await?;

    // Download natives
    download_native_libraries(manifest, &minecraft_folder).await?;

    // Download assets
    download_assets(manifest, &minecraft_folder).await?;

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum VanillaUpdateError {
    #[error("Failed to download the libraries")]
    DownloadError(#[from] DownloadError),

    #[error("No artifact found for the library")]
    NoArtifact,

    #[error("Failed to write the version manifest")]
    WriteVersionManifest(#[from] std::io::Error),

    #[error("Failed to read the assets index")]
    ReadAssetsIndex(#[from] AssetsListError),

    #[error("Failed to download the asset")]
    PoolError(#[from] tokio::task::JoinError),
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

        if library.downloads.artifact.is_none()
            || (!library.rules.is_empty() && !check_libs_rules(&library))
        {
            continue;
        }

        let library = library.clone();
        let libraries_folder = libraries_folder.clone();
        joinset.spawn(async move {
            // Download library
            download_library(library, libraries_folder).await
        });
    }

    while let Some(result) = joinset.join_next().await {
        match result {
            Ok(res) => match res {
                Ok(path) => {
                    debug!("Downloaded library: {:?}", path);
                }
                Err(e) => {
                    debug!("Error downloading library: {:?}", e);
                    return Err(e.into());
                }
            },
            Err(e) => {
                debug!("Error downloading library: {:?}", e);
                return Err(e.into());
            }
        }
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

async fn download_assets_index(
    manifest: &McVersionManifest,
    minecraft_folder: impl AsRef<Path>,
) -> Result<(), VanillaUpdateError> {
    debug!("Downloading assets index");

    let assets_index = manifest.asset_index.clone();

    let assets_folder = minecraft_folder.as_ref().join("assets");

    if !assets_folder.exists() {
        debug!(
            "Assets folder does not exist, creating it at {:?}",
            assets_folder
        );
        std::fs::create_dir_all(&assets_folder).expect("Failed to create assets folder");
    } else {
        debug!("Found assets folder at {:?}", assets_folder);
    }

    let assets_index_path = assets_folder
        .join("indexes")
        .join(format!("{}.json", assets_index.id));

    // Download the assets index
    retry_download(DownloadInfo {
        path: assets_index_path,
        url: assets_index.url,
        sha1: assets_index.sha1,
        size: assets_index.size,
    })
    .await?;

    Ok(())
}

async fn download_client(
    manifest: &McVersionManifest,
    minecraft_folder: impl AsRef<Path>,
) -> Result<(), VanillaUpdateError> {
    debug!("Downloading client");

    let client = manifest.downloads.client.clone();

    let client_folder = minecraft_folder
        .as_ref()
        .join("versions")
        .join(&manifest.id);

    if !client_folder.exists() {
        debug!(
            "Client folder does not exist, creating it at {:?}",
            client_folder
        );
        std::fs::create_dir_all(&client_folder).expect("Failed to create client folder");
    } else {
        debug!("Found client folder at {:?}", client_folder);
    }

    // Download the client.jar
    let client_path = client_folder.join(format!("{}.jar", manifest.id));

    retry_download(DownloadInfo {
        path: client_path,
        sha1: client.sha1,
        size: client.size,
        url: client.url,
    })
    .await?;

    Ok(())
}

async fn download_native_libraries(
    manifest: &McVersionManifest,
    minecraft_folder: impl AsRef<Path>,
) -> Result<(), VanillaUpdateError> {
    debug!("Downloading native libraries");

    let natives_folder = minecraft_folder.as_ref().join("natives");

    if !natives_folder.exists() {
        debug!(
            "Natives folder does not exist, creating it at {:?}",
            natives_folder
        );
        std::fs::create_dir_all(&natives_folder).expect("Failed to create natives folder");
    } else {
        debug!("Found natives folder at {:?}", natives_folder);
    }

    let os = OperatingSystem::current();
    let mut joinset = JoinSet::new();

    for lib in &manifest.libraries {
        if lib.natives.is_empty() {
            continue;
        }

        let classifiers = match &lib.downloads.classifiers {
            Some(classifiers) => classifiers.clone(),
            None => continue,
        };

        let natives_map = match lib.natives.get(&os.name()) {
            Some(map) => map,
            None => continue,
        };

        let classifier = match classifiers.get(natives_map) {
            Some(classifier) => classifier.clone(),
            None => continue,
        };

        let natives_folder = natives_folder.clone();
        joinset.spawn(async move {
            // Download native library
            download_native(classifier, natives_folder).await
        });
    }

    while let Some(result) = joinset.join_next().await {
        match result {
            Ok(res) => match res {
                Ok(path) => {
                    debug!("Downloaded native library: {:?}", path);
                }
                Err(e) => {
                    debug!("Error downloading native library: {:?}", e);
                    return Err(e.into());
                }
            },
            Err(e) => {
                debug!("Error downloading native library: {:?}", e);
                return Err(e.into());
            }
        }
    }

    Ok(())
}

async fn download_native(
    classifier: ArtifactFile,
    natives_folder: impl AsRef<Path>,
) -> Result<String, VanillaUpdateError> {
    debug!("Downloading native library");

    let file_name = classifier.path.split('/').last().unwrap();
    let native_path = natives_folder.as_ref().join(&file_name);

    // Download the native library
    retry_download(DownloadInfo {
        path: native_path,
        sha1: classifier.artifact.sha1,
        size: classifier.artifact.size,
        url: classifier.artifact.url,
    })
    .await?;

    Ok(classifier.path)
}

fn check_libs_rules(library: &Library) -> bool {
    let os = OperatingSystem::current();
    let mut allowed = false;

    for rule in &library.rules {
        let action: bool = rule.action.clone().into();

        for (name, value) in &rule.os {
            match name.as_str() {
                "name" => {
                    if value != os.name().as_ref() {
                        allowed = action;
                    }
                }
                "arch" => {
                    if value != os.arch() {
                        allowed = action;
                    }
                }
                _ => {}
            }
        }
    }

    allowed
}

async fn download_assets(
    manifest: &McVersionManifest,
    minecraft_folder: impl AsRef<Path>,
) -> Result<(), VanillaUpdateError> {
    debug!("Downloading assets");

    // Read assets from assets index
    let assets_index_path = minecraft_folder
        .as_ref()
        .join("assets")
        .join("indexes")
        .join(format!("{}.json", manifest.asset_index.id));

    let assets_index = AssetsList::from_file(&assets_index_path)?;

    let assets_folder = minecraft_folder.as_ref().join("assets");

    if !assets_folder.exists() {
        debug!(
            "Assets folder does not exist, creating it at {:?}",
            assets_folder
        );
        std::fs::create_dir_all(&assets_folder).expect("Failed to create assets folder");
    } else {
        debug!("Found assets folder at {:?}", assets_folder);
    }

    let mut joinset = JoinSet::new();

    for (name, asset) in &assets_index.objects {
        let name = name.clone();
        let asset = asset.clone();
        let assets_folder = assets_folder.clone();
        joinset.spawn(async move {
            // Download asset
            download_asset(&name, asset, assets_folder).await
        });
    }

    while let Some(result) = joinset.join_next().await {
        match result {
            Ok(res) => match res {
                Ok(_) => {
                    debug!("Downloaded asset");
                }
                Err(e) => {
                    debug!("Error downloading asset: {:?}", e);
                    return Err(e.into());
                }
            },
            Err(e) => {
                debug!("Error downloading asset: {:?}", e);
                return Err(e.into());
            }
        }
    }

    Ok(())
}

async fn download_asset(
    name: &str,
    asset_object: AssetObject,
    minecraft_folder: impl AsRef<Path>,
) -> Result<(), VanillaUpdateError> {
    debug!("Downloading asset: {}", name);

    let asset_folder = minecraft_folder.as_ref().join("objects");

    if !asset_folder.exists() {
        debug!(
            "Objects folder does not exist, creating it at {:?}",
            asset_folder
        );
        std::fs::create_dir_all(&asset_folder).expect("Failed to create objects folder");
    } else {
        debug!("Found objects folder at {:?}", asset_folder);
    }

    let asset_path = asset_folder.join(&name);

    // Download the asset
    retry_download(DownloadInfo {
        path: asset_path,
        sha1: asset_object.hash.to_string(),
        size: asset_object.size,
        url: format!(
            "{}/{}/{}",
            constants::RESOURCES_BASE,
            &asset_object.hash[..2],
            &asset_object.hash
        ),
    })
    .await?;

    Ok(())
}
