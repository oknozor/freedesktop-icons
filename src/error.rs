use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unable to parse file")]
    ParseError(#[from] ini::Error),
    #[error("IO error")]
    IOError(#[from] std::io::Error),
}