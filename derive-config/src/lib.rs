#![allow(clippy::multiple_crate_versions)]

use std::{marker::Sized, path::PathBuf};

pub use derive_macros::*;
#[cfg(feature = "dirs")]
pub use dirs::{self};
use duplicate::duplicate_item;
#[cfg(feature = "json")]
pub use json::{self};
use thiserror::Error;
#[cfg(feature = "toml")]
pub use toml::{self};
#[cfg(feature = "yaml")]
pub use yaml::{self};

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

    #[cfg(feature = "yaml")]
    #[error("{0}")]
    Yaml(#[from] yaml::Error),
}

#[duplicate_item(
    language_struct_name;
    [ DeriveJsonConfig ];
    [ DeriveTomlConfig ];
    [ DeriveYamlConfig ];
)]
pub trait language_struct_name {
    /// # Errors
    /// Will return `Err` if `dirs::config_dir` fails.
    fn path() -> Result<PathBuf, ConfigError>;

    /// # Errors
    /// Will return `Err` if `Self::path`, `File::open`, `format::to_string`, or `File::write_all` fails.
    fn save(&self) -> Result<(), ConfigError>;

    /// # Errors
    /// Will return `Err` if `Self::path`, `File::open`, `format::to_string`, or `File::write_all` fails.
    fn and_save(self) -> Result<Self, ConfigError>
    where
        Self: Sized;

    /// # Errors
    /// Will return `Err` if `Self::path`, `File::open`, `File::read_to_string`, `File::rewind`, or `format::from_str` fails.
    fn load() -> Result<Self, ConfigError>
    where
        Self: Sized;
}
