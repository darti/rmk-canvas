use std::fs;

use rmk_format::{format::NotebookBuilder, notebook};

#[test]
fn test_version_6() -> anyhow::Result<()> {
    let bytes = fs::read(
        "../data/version_6/5284170b-392b-46a7-920f-35dbce01688c/ef92ea4f-3ea9-4cdc-8981-a3fc08a97482.rm",
    )?;

    let notebook = notebook(&bytes)?;

    assert_eq!(
        notebook,
        NotebookBuilder::default().version(6).build().unwrap()
    );

    Ok(())
}

#[test]
fn test_version_5() -> anyhow::Result<()> {
    let bytes = fs::read(
        "../data/version_5/0d9af7de-39f8-4251-8500-330eec0d00f0/65b0a9a0-4019-4640-868b-5b4cc3101396.rm",
    )?;

    let notebook = notebook(&bytes)?;

    assert_eq!(
        notebook,
        NotebookBuilder::default().version(5).build().unwrap()
    );

    Ok(())
}
