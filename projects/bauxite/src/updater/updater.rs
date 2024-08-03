use tracing::debug;

use crate::{instance::Instance, updater::vanilla::update_vanilla};

use super::vanilla::VanillaUpdateError;

pub struct Updater {
    instance: Instance,
}

impl Updater {
    pub fn new(instance: Instance) -> Self {
        Updater { instance }
    }

    pub async fn update(&self) -> Result<(), UpdaterError> {
        debug!("Updating instance: {:?}", self.instance.mc_version().name());
        // TODO: clean unwanted files

        // TODO: download Minecraft files
        update_vanilla(self.instance.mc_version(), self.instance.output_dir()).await?;

        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum UpdaterError {
    #[error("Failed to download the game")]
    DownloadError(#[from] VanillaUpdateError),
}
