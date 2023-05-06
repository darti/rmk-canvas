use std::fs;

use log::info;
use rmk_format::notebook;

pub fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();
    {
        let bytes = fs::read(
            "data/5284170b-392b-46a7-920f-35dbce01688c/ef92ea4f-3ea9-4cdc-8981-a3fc08a97482.rm",
        )?;

        let notebook = notebook(&bytes)?;

        info!("{:#?}", notebook);
    }

    Ok(())
}
