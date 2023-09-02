use jre_downloader::{JreDownloader, java::{manifest::RequestType, download::download_manager::jre_callback::{DownloadCallback, DownloadInfos, DownloadStep}}};

#[derive(Copy, Clone)]
struct MyDownloadCallback;

impl DownloadCallback for MyDownloadCallback {
    fn on_start(&self) {
        println!("Download started");
    }

    fn on_progress(&self, infos: DownloadInfos) {
        //println!("Download progress: {}/{}", infos.get_total_files_downloaded(), infos.get_total_files_to_download());
        println!("Download progress: {}/{}", infos.get_total_bytes_downloaded(), infos.get_total_bytes_to_download());
    }

    fn on_file_downloaded(&self, file_name: String) {
        println!("File {} downloaded", file_name);
    }

    fn on_end(&self) {
        println!("Download ended");
    }

    fn on_step(&self, step: DownloadStep) {
        println!("Step: {:?}", step);
    }
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    // Download the latest JRE
    JreDownloader::download_jre(RequestType::JreLegacy, "./java/".to_string(), MyDownloadCallback)
        .await
        .expect("Failed to download JRE");
}
