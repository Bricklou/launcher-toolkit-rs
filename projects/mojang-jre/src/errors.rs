pub type JreResult<T> = Result<T, JreError>;

#[derive(Debug, thiserror::Error)]

pub enum JreError {
    #[error("Unsupported combo OS/Architecture detected: {0}/{1}")]
    UnsupportedPlatform(String, String),

    #[error("There was an error while reading the manifest file")]
    ManifestReadError,

    #[error("There was an error while parsing the manifest file")]
    ManifestParseError,

    #[error("There was an error while downloading the JRE")]
    DownloadError(#[from] reqwest::Error),

    #[error("No runtime available for this platform")]
    NoRuntimeAvailable,

    #[error("There was an IO error")]
    IOError(#[from] std::io::Error),

    #[error("There was an error while calculating the hash {0}")]
    HashError(#[from] hex::FromHexError),

    #[error("The checksum of the file is invalid")]
    InvalidChecksum,
}
