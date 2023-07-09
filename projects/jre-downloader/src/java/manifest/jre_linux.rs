use super::manifest_obj::JreManifest;

impl JreManifest {
    #[cfg(target_os = "linux")]
    pub async fn get_runtime_url(
        &self,
        request_type: super::RequestType,
    ) -> crate::error::JreResult<String> {
        // Import stuff
        use super::{manifest_obj::RuntimeData, RequestType};
        use crate::error::JreError;
        use std::collections::HashMap;

        let data: HashMap<RequestType, Vec<RuntimeData>>;

        // Get the data array for a specific request type
        cfg_if::cfg_if! {
            if #[cfg(target_arch = "x86_64")] {
                data = self.linux.clone();
            } else if #[cfg(target_arch = "x86")] {
                data = self.linux_i386.clone();
            } else {
                compile_error!("Unsupported architecture");
                return Err(JreError::NoRuntimeAvailable);
            }
        }
        

        // Get the data array for a specific request type
        if let Some(data) = data.get(&request_type) {
            // Is there a value ?
            if let Some(json_result) = data.get(0) {
                // Get the first element of the array
                return Ok(json_result.manifest.url.clone());
            }
        }

        // Lol no
        Err(JreError::NoRuntimeAvailable)
    }
}
