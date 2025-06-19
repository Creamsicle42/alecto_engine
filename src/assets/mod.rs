use eyre::Result;
use std::fs::File;

mod archive_manager;

#[derive(Debug)]
pub struct AssetManager {
    archive_manager: archive_manager::ArchiveManager<File>,
}

impl AssetManager {
    pub fn new(archives: Vec<File>) -> Result<Self> {
        let archive_manager = archive_manager::ArchiveManager::create(archives)?;
        Ok(AssetManager { archive_manager })
    }
}
