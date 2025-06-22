#![allow(unused)]
use eyre::{Result, eyre};
use std::{collections::HashMap, io::prelude::*};
use zip::ZipArchive;

#[derive(Debug)]
struct ArchiveResourceIndex {
    file_index: usize,
    archive_index: usize,
}

#[derive(Debug, Default)]
pub struct ArchiveManager<R: Read + Seek> {
    archives: Vec<ZipArchive<R>>,
    resource_map: HashMap<String, ArchiveResourceIndex>,
}

impl<R: Read + Seek> ArchiveManager<R> {
    pub fn create(archive_files: Vec<R>) -> Result<ArchiveManager<R>> {
        let mut archives: Vec<_> = vec![];

        for file in archive_files.into_iter() {
            archives.push(ZipArchive::new(file)?);
        }

        let mut resource_map: HashMap<String, ArchiveResourceIndex> = HashMap::new();
        for (archive_index, archive) in archives.iter_mut().enumerate() {
            for file_index in 0..archive.len() {
                let file_name = archive
                    .by_index(file_index)?
                    .enclosed_name()
                    .ok_or(eyre!(
                        "File in zip archive has invalid name, file path out of bounds."
                    ))?
                    .to_str()
                    .ok_or(eyre!(
                        "File in zip archive has invalid name, file path contains invalid bytes."
                    ))?
                    .to_owned();
                log::debug!("Indexing asset file \"{}\"", file_name);
                resource_map.insert(
                    file_name,
                    ArchiveResourceIndex {
                        file_index,
                        archive_index,
                    },
                );
            }
        }
        Ok(ArchiveManager {
            archives,
            resource_map,
        })
    }

    pub fn get_resource_bytes(&mut self, resource_id: String) -> Option<Vec<u8>> {
        let location_info = self.resource_map.get(&resource_id)?;
        let archive = self.archives.get_mut(location_info.archive_index)?;
        let mut file = archive.by_index(location_info.file_index).ok()?;
        let mut buf: Vec<u8> = vec![];
        file.read_to_end(&mut buf).ok()?;
        Some(buf)
    }
}
