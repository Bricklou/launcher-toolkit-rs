use std::path::Path;

use tracing::info;

#[derive(Debug)]
pub enum DownloadStep {
    Manifest,
    Checking,
    Downloading,
    Done,
}

#[derive(Debug)]
pub enum DownloadFileStep {
    Downloading { current: u64, total: u64 },
    Linking,
    Checking,
    Done,
}

pub trait DownloadCallback: Sync + Send {
    fn on_start(&self);
    fn on_step(&self, step: DownloadStep);
    fn on_file_step(&self, path: &Path, step: DownloadFileStep);
    fn on_file_downloaded(&self, path: &Path, downloaded: u64, total: u64);
    fn on_finish(&self);
}

#[derive(Debug, Default)]
pub struct DefaultDownloadCallback;

impl DownloadCallback for DefaultDownloadCallback {
    fn on_start(&self) {
        info!("Download started");
    }

    fn on_step(&self, step: DownloadStep) {
        info!("Step: {:?}", step);
    }

    fn on_file_step(&self, path: &Path, step: DownloadFileStep) {
        info!("Step: {:?} for file: {:?}", step, path);
    }

    fn on_file_downloaded(&self, path: &Path, downloaded: u64, total: u64) {
        info!(
            "Downloaded: {} / {} bytes for file: {:?}",
            downloaded, total, path
        );
    }

    fn on_finish(&self) {
        info!("Download finished");
    }
}
