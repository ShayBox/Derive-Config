#![allow(clippy::multiple_crate_versions)]

use std::{marker::Sized, path::PathBuf};

pub use derive_macros::*;
#[cfg(feature = "directories")]
pub use directories::{self};
#[cfg(feature = "dirs")]
pub use dirs::{self};
#[cfg(feature = "etcetera")]
pub use etcetera::{self};
#[cfg(feature = "json")]
pub use json::{self};
#[cfg(feature = "toml")]
pub use toml::{self};
#[cfg(feature = "yaml")]
pub use yaml::{self};

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("None")]
    None,

    #[cfg(feature = "etcetera")]
    #[error(transparent)]
    HomeDir(#[from] etcetera::HomeDirError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[cfg(feature = "json")]
    #[error(transparent)]
    Json(#[from] json::Error),

    #[cfg(feature = "toml")]
    #[error(transparent)]
    TomlDe(#[from] toml::de::Error),

    #[cfg(feature = "toml")]
    #[error(transparent)]
    TomlSer(#[from] toml::ser::Error),

    #[cfg(feature = "yaml")]
    #[error(transparent)]
    Yaml(#[from] yaml::Error),
}

#[duplicate::duplicate_item(
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
