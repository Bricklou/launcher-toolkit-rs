use std::collections::HashMap;

use tracing::debug;

use crate::errors::{JreError, JreResult};

use super::{JreManifestExt, RuntimeData, VersionType, VersionsManifest};

impl JreManifestExt for VersionsManifest {
    fn get_runtime_url(&self, version_type: &VersionType) -> JreResult<String> {
        debug!(message = "Getting the runtime URL", version_type = ?version_type);

        let data: HashMap<VersionType, Vec<RuntimeData>>;

        #[cfg(target_arch = "x86_64")]
        {
            data = self.linux.clone();
        }
        #[cfg(target_arch = "x86")]
        {
            data = self.linux_i386.clone();
        }
        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
        {
            compile_error!("Unsupported architecture");
            return Err(JreError::NoRuntimeAvailable);
        }

        // Get the data array for a specific version type
        if let Some(data) = data.get(&version_type) {
            // Is there a value?
            if let Some(json_result) = data.get(0) {
                // Get the first element of the array
                return Ok(json_result.manifest.url.clone());
            }
        }

        // If we get here, there was no data available
        Err(JreError::NoRuntimeAvailable)
    }
}
