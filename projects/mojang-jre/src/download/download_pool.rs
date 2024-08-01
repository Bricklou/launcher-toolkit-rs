use futures_util::StreamExt;
use sha1::{Digest, Sha1};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::io::AsyncWriteExt;
use tracing::debug;

use crate::{
    callback::{DownloadCallback, DownloadFileStep},
    errors::JreResult,
    jre::{FileType, JreFile},
};

pub async fn retry_download_item(
    path: PathBuf,
    file: JreFile,
    callback: Arc<dyn DownloadCallback>,
) -> std::io::Result<PathBuf> {
    for _ in 0..5 {
        match download_item(path.clone(), file.clone(), callback.clone()).await {
            Ok(path) => return Ok(path),
            Err(e) => {
                debug!("Error downloading file: ({:?}) {:?}", path, e);

                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        }
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Failed to download the file",
    ))
}

pub async fn download_item(
    path: PathBuf,
    file: JreFile,
    callback: Arc<dyn DownloadCallback>,
) -> JreResult<PathBuf> {
    match file.file_type {
        FileType::Directory => {
            debug!("Creating directory: {:?}", path);
            if !path.exists() {
                callback.on_file_step(&path, DownloadFileStep::Checking);
                tokio::fs::create_dir_all(&path).await?;

                callback.on_file_step(&path, DownloadFileStep::Done);
            }
        }
        FileType::Link => {
            debug!("Creating symlink: {:?}", path);
            callback.on_file_step(&path, DownloadFileStep::Linking);

            let target = file.target.unwrap();
            std::os::unix::fs::symlink(&target, &path)?;

            callback.on_file_step(&path, DownloadFileStep::Done);
        }
        FileType::File => {
            debug!("Checking if the file exists: {:?}", path);
            callback.on_file_step(&path, DownloadFileStep::Checking);
            let download_file = file.downloads.unwrap();

            let raw_file = download_file.raw;

            if path.exists() {
                debug!("Checking file hash: {:?}", path);

                match check_file_hash(&path, &raw_file.sha1, &callback).await {
                    Ok(_) => {
                        debug!("File hash is correct: {:?}", path);

                        callback.on_file_step(&path, DownloadFileStep::Done);
                        return Ok(path);
                    }
                    Err(_) => {
                        debug!("File hash is incorrect: {:?}", path);
                        // Delete the file
                        tokio::fs::remove_file(&path).await?;
                    }
                }
            }

            callback.on_file_step(
                &path,
                DownloadFileStep::Downloading {
                    current: 0,
                    total: raw_file.size as u64,
                },
            );
            debug!("Downloading file: {:?}", path);

            let mut file = tokio::fs::File::create(&path).await?;
            let response = reqwest::get(&raw_file.url).await?;
            let mut content = response.bytes_stream();

            let mut downloaded = 0;

            while let Some(item) = content.next().await {
                let item = item?;
                downloaded += item.len() as u64;

                file.write_all(&item).await?;

                callback.on_file_step(
                    &path,
                    DownloadFileStep::Downloading {
                        current: downloaded,
                        total: raw_file.size as u64,
                    },
                );
            }

            debug!("File downloaded: {:?}", path);

            debug!("Checking file hash: {:?}", path);

            match check_file_hash(&path, &raw_file.sha1, &callback).await {
                Ok(_) => {
                    callback.on_file_step(&path, DownloadFileStep::Done);
                    debug!("File hash is correct: {:?}", path);
                }
                Err(_) => {
                    debug!("File hash is incorrect: {:?}", path);
                    // Delete the file
                    tokio::fs::remove_file(&path).await?;
                }
            }
        }
    }

    Ok(path)
}

async fn check_file_hash(
    file_path: &Path,
    hash: &String,
    callback: &Arc<dyn DownloadCallback>,
) -> JreResult<bool> {
    callback.on_file_step(file_path, DownloadFileStep::Checking);
    debug!("Checking file hash: {:?}", file_path);
    // Get the remote hash
    let remote_sha = hex::decode(hash)?;

    // Get the file hash
    let mut file = std::fs::File::open(file_path)?;
    let mut hasher = Sha1::new();
    std::io::copy(&mut file, &mut hasher)?;
    let hash = hasher.finalize();

    if hash.as_slice() == remote_sha.as_slice() {
        debug!("File hash is correct: {:?}", file_path);
        return Ok(true);
    }

    Ok(false)
}
