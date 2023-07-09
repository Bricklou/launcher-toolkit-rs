use sha1::{Digest, Sha1};
use std::{path::Path, fs::{File, remove_file}, io::Write};

use futures_util::StreamExt;


use crate::{error::JreResult, java::JreFile};

use self::jre_callback::DownloadCallback;


pub mod jre_callback{
    use std::sync::atomic::{AtomicUsize};
    static mut TOTAL_BYTES_TO_DOWNLOAD: AtomicUsize = AtomicUsize::new(0);
    static mut TOTAL_BYTES_DOWNLOADED: AtomicUsize = AtomicUsize::new(0);
    static mut TOTAL_FILES_TO_DOWNLOAD: AtomicUsize = AtomicUsize::new(0);
    static mut TOTAL_FILES_DOWNLOADED: AtomicUsize = AtomicUsize::new(0);

    pub fn init(bytes_to_download: usize, files_to_download: usize){
        unsafe {
            TOTAL_BYTES_TO_DOWNLOAD = AtomicUsize::new(bytes_to_download);
            TOTAL_BYTES_DOWNLOADED = AtomicUsize::new(0);
            TOTAL_FILES_TO_DOWNLOAD = AtomicUsize::new(files_to_download);
            TOTAL_FILES_DOWNLOADED = AtomicUsize::new(0);
        }
    }

    pub fn get_download_infos() -> DownloadInfos {
        unsafe {
            DownloadInfos{
                total_bytes_to_download: TOTAL_BYTES_TO_DOWNLOAD.load(std::sync::atomic::Ordering::Relaxed),
                total_bytes_downloaded: TOTAL_BYTES_DOWNLOADED.load(std::sync::atomic::Ordering::Relaxed),
                total_files_to_download: TOTAL_FILES_TO_DOWNLOAD.load(std::sync::atomic::Ordering::Relaxed),
                total_files_downloaded: TOTAL_FILES_DOWNLOADED.load(std::sync::atomic::Ordering::Relaxed),
            }
        }
    }

    pub fn add_bytes_downloaded(bytes: usize){
        unsafe {
            TOTAL_BYTES_DOWNLOADED.fetch_add(bytes, std::sync::atomic::Ordering::Relaxed);
        }
    }

    pub fn add_file_downloaded(){
        unsafe {
            TOTAL_FILES_DOWNLOADED.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
    }


    pub struct DownloadInfos{
        total_bytes_to_download: usize,
        total_bytes_downloaded: usize,
        total_files_to_download: usize,
        total_files_downloaded: usize,
    }

    impl DownloadInfos{
        pub fn get_total_bytes_to_download(&self) -> usize{
            self.total_bytes_to_download
        }

        pub fn get_total_bytes_downloaded(&self) -> usize{
            self.total_bytes_downloaded
        }

        pub fn get_total_files_to_download(&self) -> usize{
            self.total_files_to_download
        }

        pub fn get_total_files_downloaded(&self) -> usize{
            self.total_files_downloaded
        }
    }

    #[derive(Debug)]
    pub enum DownloadStep {
        INIT,
        MANIFEST,
        CHECKING,
        DOWNLOADING,
        END,
    }
    
    /**
     * This trait provides useful methods to implement to access to download and update status.
     */
    pub trait DownloadCallback {
        fn on_start(&self);
        fn on_progress(&self, infos: DownloadInfos);
        fn on_step(&self, step: DownloadStep);
        fn on_file_downloaded(&self, file_name: String);
        fn on_end(&self);
    }
    
}


pub async fn download_file<P: AsRef<Path>>(file_path: P, file_data: JreFile, download_callback: impl DownloadCallback) -> JreResult<()> {
    let file_path = file_path.as_ref();
    let raw_data = file_data.downloads.unwrap().raw;

    let mut file = File::create(&file_path)?;
    let response = reqwest::get(&raw_data.url).await?;
    let mut content = response.bytes_stream();
    while let Some(item) = content.next().await {
        let item = item?;
        file.write_all(&item)?;
        jre_callback::add_bytes_downloaded(item.len());
        download_callback.on_progress(jre_callback::get_download_infos());
    }
    

    download_callback.on_file_downloaded(file_path.to_str().unwrap().to_string());
    jre_callback::add_file_downloaded();
    download_callback.on_progress(jre_callback::get_download_infos());
    Ok(())
}

pub fn check_file<P: AsRef<Path>>(file_path: P, file_data: JreFile) -> JreResult<bool> {
    let file_path = file_path.as_ref();
    let raw_data = file_data.downloads.unwrap().raw;
    if file_path.exists() {
        let remote_sha = hex::decode(&raw_data.sha1)?;
        let mut file = File::open(&file_path)?;
        let mut hasher = Sha1::new();
        std::io::copy(&mut file, &mut hasher)?;
        let hash = hasher.finalize();

        if hash.as_slice() == remote_sha.as_slice() {
            println!("File {:?} already exists and is up to date", file_path);
            return Ok(true);
        }
        remove_file(file_path)?;
    }
    Ok(false)
}