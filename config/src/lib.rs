use std::{marker::Sized, path::PathBuf};

pub use derive::*;
pub use dirs::{self};
pub use eyre::{self};
pub use toml::{self};

pub trait ConfigFile {
    /// # Errors
    ///
    /// Will return `Err` if `dirs::config_dir` returns err.
    fn path() -> eyre::Result<PathBuf>;

    /// # Errors
    fn save(&self) -> eyre::Result<()>;

    /// # Errors
    fn load() -> eyre::Result<Self>
    where
        Self: Sized;
}
