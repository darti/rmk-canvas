use crate::{
    errors::RmkFormatError,
    format::{Notebook, NotebookBuilder},
};

pub fn parse(i: &[u8]) -> Result<Notebook, RmkFormatError> {
    NotebookBuilder::default()
        .version(6)
        .nb_layers(0)
        .build()
        .map_err(|e| e.into())
}
