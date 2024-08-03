use std::path::{Path, PathBuf};

use crate::minecraft::version::MinecraftVersion;

pub struct InstanceBuilder {
    output_dir: Option<PathBuf>,
    mc_version: Box<dyn MinecraftVersion + 'static>,
}

impl InstanceBuilder {
    pub fn new<V: MinecraftVersion + 'static>(version: V) -> Self {
        InstanceBuilder {
            output_dir: None,
            mc_version: Box::new(version),
        }
    }

    pub fn with_output_dir(mut self, output_dir: impl Into<PathBuf>) -> Self {
        self.output_dir = Some(output_dir.into());
        self
    }

    pub fn build(self) -> Instance {
        Instance {
            output_dir: self.output_dir.unwrap(),
            mc_version: self.mc_version,
        }
    }
}

pub struct Instance {
    output_dir: PathBuf,
    mc_version: Box<dyn MinecraftVersion>,
}

impl Instance {
    pub fn output_dir(&self) -> &Path {
        &self.output_dir
    }

    pub fn mc_version(&self) -> &dyn MinecraftVersion {
        self.mc_version.as_ref()
    }
}
