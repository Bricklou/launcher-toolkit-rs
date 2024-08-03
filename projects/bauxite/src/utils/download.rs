use std::path::PathBuf;

use futures_util::StreamExt;
use sha1::{Digest, Sha1};
use tokio::io::AsyncWriteExt;
use tracing::debug;

#[derive(Clone, Debug)]
pub struct DownloadInfo {
    pub path: PathBuf,
    pub url: String,
    pub size: u64,
    pub sha1: String,
}

#[derive(thiserror::Error, Debug)]
pub enum DownloadError {
    #[error("Failed to download the file")]
    HttpError(#[from] reqwest::Error),

    #[error("Failed to write the file")]
    IoError(#[from] std::io::Error),

    #[error("There was an error while calculating the hash {0}")]
    HashError(#[from] hex::FromHexError),

    #[error("The checksum of the file is invalid")]
    InvalidChecksum,

    #[error("Failed to download the file after 5 retries")]
    DownloadError,
}

pub async fn retry_download(download_info: DownloadInfo) -> Result<PathBuf, DownloadError> {
    for _ in 0..5 {
        match download_item(download_info.clone()).await {
            Ok(path) => return Ok(path),
            Err(e) => {
                debug!(
                    "Error downloading file (retry in 5 seconds): ({:?}) {:?}",
                    download_info.path, e
                );

                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        }
    }

    Err(DownloadError::DownloadError)
}

async fn download_item(download_info: DownloadInfo) -> Result<PathBuf, DownloadError> {
    debug!("Checking if the file exists: {:?}", download_info.path);

    if download_info.path.exists() {
        debug!(
            "File already exists, checking file hash: {:?}",
            download_info.path
        );

        match check_file_hash(&download_info).await {
            Ok(_) => return Ok(download_info.path),
            Err(_) => {
                debug!(
                    "File hash does not match, removing file: {:?}",
                    download_info.path
                );
                tokio::fs::remove_file(&download_info.path).await?;
            }
        }
    }

    // Check if the parent directory exists
    if let Some(parent) = download_info.path.parent() {
        if !parent.exists() {
            debug!("Parent directory does not exist, creating it: {:?}", parent);
            tokio::fs::create_dir_all(parent).await?;
        }
    }

    debug!("Downloading file: {:?}", download_info.path);

    let mut file = tokio::fs::File::create(&download_info.path).await?;
    let response = reqwest::get(&download_info.url).await?;

    let mut content = response.bytes_stream();

    let mut downloaded = 0;

    while let Some(item) = content.next().await {
        let chunk = item?;
        downloaded += chunk.len() as u64;

        file.write_all(&chunk).await?;

        debug!(
            "Downloading file: {:?} ({}/{})",
            download_info.path, downloaded, download_info.size
        );
    }

    debug!("File downloaded: {:?}", download_info.path);

    match check_file_hash(&download_info).await {
        Ok(_) => {}
        Err(_) => {
            // Delete the file
            tokio::fs::remove_file(&download_info.path).await?;
        }
    }

    Ok(download_info.path)
}

async fn check_file_hash(download_info: &DownloadInfo) -> Result<(), DownloadError> {
    debug!("Checking file hash: {:?}", download_info.path);

    // Get the remote hash
    let remote_sha = hex::decode(&download_info.sha1)?;

    // Get the file hash
    let mut file = std::fs::File::open(&download_info.path)?;
    let mut hasher = Sha1::new();
    std::io::copy(&mut file, &mut hasher)?;
    let hash = hasher.finalize();

    if hash.as_slice() == remote_sha.as_slice() {
        debug!("File hash is correct: {:?}", download_info.path);
        return Ok(());
    }

    Err(DownloadError::InvalidChecksum)
}
