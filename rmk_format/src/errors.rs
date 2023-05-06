use crate::format::NotebookBuilderError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RmkFormatError {
    #[error("build error")]
    BuildError(#[from] NotebookBuilderError),

    #[error("parser error")]
    ParserError(#[from] nom::Err<nom::error::Error<Vec<u8>>>),

    #[error("invalide version: {0}")]
    InvalidVersion(String),
}
