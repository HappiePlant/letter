use hex::FromHexError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Repository {0} not found")]
    RepoNotFound(String),

    #[error("Repo ID already exists")]
    RepoIdAlreadyExists,

    #[error("Repo url already exists")]
    RepoUrlAlreadyExists,

    #[error("Error fetching from repository")]
    RepoFetchError,

    #[error("Error fetching from repository")]
    RepoRequestError(#[from] reqwest::Error),

    #[error("Not found in repository")]
    RepoFetchNotFound,

    #[error("Provided font hash is invalid")]
    FontInvalidHash(#[from] FromHexError),

    #[error("Font has no versions or requested version could not be found")]
    FontVersionNotFound,

    #[error("Can't install both variable and static variants")]
    ArgsBothVariableStatic,

    #[error("Invalid URL was provided")]
    ArgsInvalidUrl,

    #[error("No matching files found for the specified options")]
    FontNoMatch,

    #[error("All font sources failed to download")]
    FontDownloadFailure,

    #[error("Font not found in any repo")]
    FontNotFound,

    #[error("Store IO error: {0}")]
    StoreIoError(String),

    #[error("Zip error: {0}")]
    ZipError(String),

    #[error("Glob error")]
    GlobError(#[from] globset::Error),

    #[error("TOML error")]
    TomlError(#[from] toml::ser::Error),

    #[error("Hash mismatch")]
    HashMismatch,

    #[error("Config error: {0}")]
    ConfigError(String),
}
