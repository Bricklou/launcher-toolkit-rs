use std::{
    path::{Path, PathBuf},
    sync::{atomic::AtomicUsize, Arc},
};

use tokio::task::JoinSet;
use tracing::debug;

use crate::{
    callback::{DefaultDownloadCallback, DownloadCallback, DownloadStep},
    download::download_pool::retry_download_item,
    errors::JreResult,
    jre::{VersionType, VersionsManifest},
};

pub struct MojangJre {
    version_type: VersionType,
    path: PathBuf,
    callback: Arc<dyn DownloadCallback>,
}

impl MojangJre {
    pub fn new(
        version_type: VersionType,
        path: impl AsRef<Path>,
        callback: Option<Arc<dyn DownloadCallback>>,
    ) -> Self {
        Self {
            version_type,
            path: path.as_ref().to_path_buf(),
            callback: callback.unwrap_or(Arc::new(DefaultDownloadCallback {})),
        }
    }

    pub async fn download(&mut self) -> JreResult<()> {
        self.callback.on_start();
        self.callback.on_step(DownloadStep::Manifest);

        debug!("Getting the manifest data");
        let manifest = VersionsManifest::get().await?;
        let jre_files = manifest.get_files(&self.version_type).await?;

        // Checking the files
        self.callback.on_step(DownloadStep::Checking);

        debug!("Checking if the output directory exists");
        if !self.path.exists() {
            debug!("Directory does not exist, creating it");
            std::fs::create_dir_all(&self.path)?;
        }

        let tasks = jre_files.iter().map(|(file_name, jre_file)| {
            let jre_file_path = self.path.join(&file_name);

            retry_download_item(
                jre_file_path.clone(),
                jre_file.clone(),
                self.callback.clone(),
            )
        });

        // Downloading the files
        self.callback.on_step(DownloadStep::Downloading);

        let mut joinset = JoinSet::from_iter(tasks);

        let counter = AtomicUsize::new(0);
        let total_files = jre_files.len();

        while let Some(result) = joinset.join_next().await {
            match result {
                Ok(res) => match res {
                    Ok(path) => {
                        debug!("File downloaded: {:?}", path);

                        let current = counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                        self.callback
                            .on_file_downloaded(&path, current as u64, total_files as u64);
                    }
                    Err(e) => {
                        debug!("Error downloading file: {:?}", e);
                        return Err(e.into());
                    }
                },
                Err(e) => {
                    debug!("Error downloading file: {:?}", e);
                }
            }
        }

        self.callback.on_step(DownloadStep::Done);

        Ok(())
    }
}
