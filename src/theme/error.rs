use std::io;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum ThemeError {
    #[error("No 'index.theme' file for {0}")]
    ThemeIndexNotFound(PathBuf),
    #[error("IoError: {0}")]
    IoError(#[from] io::Error),
}
