use derive_builder::Builder;

#[derive(Builder, Debug, PartialEq)]
pub struct Notebook {
    version: usize,
}
