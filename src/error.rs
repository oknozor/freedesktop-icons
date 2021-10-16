use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error")]
    ParseError(#[from] serde_ini::de::Error),
    #[error("IO error")]
    IOError(#[from] std::io::Error),
}
