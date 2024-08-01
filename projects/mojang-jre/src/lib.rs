#[cfg(not(any(
    all(
        target_os = "windows",
        any(target_arch = "x86", target_arch = "x86_64", target_arch = "arm")
    ),
    all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64")),
    all(target_os = "macos", any(target_arch = "x86_64", target_arch = "arm"))
)))]
compile_error!("Unsupported platform");

mod constants;
mod download;
mod errors;
pub mod jre;
mod mojang_jre;

pub use mojang_jre::*;
pub mod callback;
