use eyre::{Result, WrapErr};
use std::path::PathBuf;

/// Things that implement AsPath can behave like paths.
pub trait AsPath {
    /// Return a PathBuf that represents self as a path relative path to the
    /// current directory.
    fn as_relative(&self) -> Result<PathBuf>;
}

impl AsPath for &str {
    fn as_relative(&self) -> Result<PathBuf> {
        let base_path =
            std::env::current_dir().wrap_err("failed to determine the current directory")?;
        Ok(base_path.join(self))
    }
}
