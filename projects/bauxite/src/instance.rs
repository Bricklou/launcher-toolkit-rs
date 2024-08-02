use std::sync::Arc;

use crate::minecraft::version::MinecraftVersion;

pub struct InstanceBuilder {
    output_dir: Option<String>,
    mc_version: Option<Arc<dyn MinecraftVersion>>,
}

impl InstanceBuilder {
    pub fn new() -> Self {
        InstanceBuilder {
            output_dir: None,
            mc_version: None,
        }
    }

    pub fn with_output_dir(mut self, output_dir: &str) -> Self {
        self.output_dir = Some(output_dir.to_string());
        self
    }

    pub fn with_vanilla_version(mut self, version: Arc<dyn MinecraftVersion>) -> Self {
        self.mc_version = Some(version);
        self
    }

    pub fn build(self) -> Instance {
        Instance {
            output_dir: self.output_dir.unwrap(),
        }
    }
}

pub struct Instance {
    output_dir: String,
}
