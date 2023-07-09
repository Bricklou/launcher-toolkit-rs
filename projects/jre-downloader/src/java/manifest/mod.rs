mod jre_mac;
mod jre_linux;
mod jre_windows;

mod manifest_obj;
pub use manifest_obj::*;

pub use manifest_obj::RequestType;

pub struct JreManifestGetter {}