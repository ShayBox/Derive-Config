use std::{marker::Sized, path::PathBuf};

pub use derive_macros::*;
pub use dirs::{self};
#[cfg(feature = "json")]
pub use json::{self};
use thiserror::Error;
#[cfg(feature = "toml")]
pub use toml::{self};

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("None")]
    None,

    #[error("{0}")]
    Io(#[from] std::io::Error),

    #[cfg(feature = "json")]
    #[error("{0}")]
    Json(#[from] json::Error),

    #[cfg(feature = "toml")]
    #[error("{0}")]
    TomlDe(#[from] toml::de::Error),

    #[cfg(feature = "toml")]
    #[error("{0}")]
    TomlSer(#[from] toml::ser::Error),
}

pub trait ConfigFile {
    /// # Errors
    ///
    /// Will return `Err` if `dirs::config_dir` returns err.
    fn path() -> Result<PathBuf, ConfigError>;

    /// # Errors
    fn save(&self) -> Result<(), ConfigError>;

    /// # Errors
    fn load() -> Result<Self, ConfigError>
    where
        Self: Sized;
}
