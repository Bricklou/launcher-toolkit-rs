mod constants;
pub mod error;
pub mod java;
use std::{collections::HashMap, path::Path, os::unix::prelude::PermissionsExt};

use java::{
    download::download_manager::{
        check_file,
        jre_callback::{init, DownloadCallback, DownloadStep},
    },
    manifest::{JreManifest, RequestType},
    JreFile,
};
#[cfg(not(any(
    all(
        target_os = "windows",
        any(target_arch = "x86", target_arch = "x86_64", target_arch = "arm")
    ),
    all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64")),
    all(target_os = "macos", any(target_arch = "x86_64", target_arch = "arm"))
)))]
compile_error!("Unsupported platform");

#[cfg(unix)]
use std::os::unix::fs::symlink;

use crate::java::FileType;

pub struct JreDownloader;

impl JreDownloader {
    pub async fn download_jre<P: AsRef<Path>>(
        jre_type: RequestType,
        path: P,
        download_callback: impl DownloadCallback + std::marker::Copy,
    ) -> error::JreResult<()> {
        let path = path.as_ref();

        download_callback.clone().on_start();
        download_callback.clone().on_step(DownloadStep::INIT);
        download_callback.clone().on_step(DownloadStep::MANIFEST);

        //Getting the manifest data
        let manifest = JreManifest::get().await?;
        let jre_url = manifest.get_runtime_url(jre_type).await?;
        let jre_files = crate::java::get_jre_files(&jre_url).await?;

        //Checking the files
        download_callback.clone().on_step(DownloadStep::CHECKING);
        let mut total_files: usize = 0;
        let mut total_bytes: usize = 0;
        if !path.exists() {
            let _ = std::fs::create_dir_all(path);
        }

        jre_files.iter().for_each(|file| {
            if file.1.file_type == FileType::Directory {
                let path_name = path.join(file.0);
                if !path_name.exists() {
                    let _ = std::fs::create_dir_all(path_name);
                }
            }
        });

        let mut files_to_download: HashMap<String, JreFile> = HashMap::new();
        

        jre_files.iter().for_each(|file| {
            if file.1.file_type == FileType::File {
                let path_name = path.join(file.0.clone());
                let check = check_file(path_name.clone(), file.1.clone());
                if check.is_err() {
                    println!("Error checking file: {:?}", check);
                    panic!("Error checking file: {:?}", check);
                }
                if !check.unwrap() {
                    total_bytes += file.1.downloads.as_ref().unwrap().raw.size;
                    total_files += 1;
                    files_to_download.insert(file.0.clone(), file.1.clone());
                }
            }
        });

        init(total_bytes, total_files);
        

        //start the download
        download_callback.clone().on_step(DownloadStep::DOWNLOADING);

        //loop in all of the files and create the folder if it doesn't exist
        for file in files_to_download {
            let path_name = path.join(file.0.clone());
            match file.1.file_type.clone() {
                crate::java::FileType::File => {
                    let _ = crate::java::download::download_manager::download_file(
                        path_name.clone(),
                        file.1.clone(),
                        download_callback.clone(),
                    )
                    .await; //download_file(&download.raw.url, &path_name).await?;
                }
                crate::java::FileType::Link => {
                    let path_name = path.join(file.0.clone());
                    if !path_name.exists() {
                        println!("Creating symlink {:?} to {:?}", file.0, file.1.target);
                        #[cfg(unix)]
                        let _ = symlink(file.1.target.unwrap(), path_name);
                    }
                }
                _ => continue,
            }
            #[cfg(unix)]
            if Some(true) == file.1.executable {
                let _ = std::fs::set_permissions(path_name, std::fs::Permissions::from_mode(0o755));
            }
        }
        download_callback.on_step(DownloadStep::END);
        Ok(())
    }
}
