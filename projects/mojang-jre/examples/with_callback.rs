use std::{path::Path, sync::Arc, time::Duration};

use indicatif::{ProgressBar, ProgressStyle};
use mojang_jre::{
    callback::{DownloadCallback, DownloadFileStep, DownloadStep},
    jre::VersionType,
    MojangJre,
};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

struct MyCallback {
    pb: ProgressBar,
}

impl MyCallback {
    pub fn new() -> Self {
        let pb = ProgressBar::new(100);
        pb.set_style(
            ProgressStyle::with_template(
                "[{elapsed_precise}] {bar:80.cyan/blue} {pos:>7}/{len:7} {msg}",
            )
            .unwrap()
            .progress_chars("##-"),
        );
        Self { pb }
    }
}

impl DownloadCallback for MyCallback {
    fn on_start(&self) {
        self.pb.set_message("Starting download");
        self.pb.enable_steady_tick(Duration::from_millis(100));
        self.pb.reset();
    }

    fn on_step(&self, step: DownloadStep) {
        let txt = match step {
            DownloadStep::Manifest => "Getting the manifest data",
            DownloadStep::Checking => "Checking the files",
            DownloadStep::Downloading => "Downloading the files",
            DownloadStep::Done => "Finished",
        };
        self.pb.set_message(txt);
    }

    fn on_file_step(&self, _path: &Path, _step: DownloadFileStep) {}

    fn on_file_downloaded(&self, path: &Path, downloaded: u64, total: u64) {
        self.pb.set_length(total);
        self.pb.set_position(downloaded);
        self.pb.set_message(path.to_string_lossy().to_string());
    }

    fn on_finish(&self) {}
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer().pretty())
        .with(EnvFilter::from_default_env())
        .init();

    let callback = Arc::new(MyCallback::new());

    let mut jre = MojangJre::new(VersionType::Legacy, "./tmp", Some(callback));

    jre.download().await.expect("Failed to download JRE");
}
