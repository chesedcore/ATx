use log::{info, error};
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::{Path, PathBuf};
use project_root::get_project_root;

const CHUNK_SIZE: usize = 1024;

#[derive(Debug)]
pub struct Loader {
    pub root: PathBuf,
    pub raw: PathBuf,
    pub from: PathBuf,
}

impl Loader {
    pub fn new() -> Result<Self, String> {
        let root = get_project_root().map_err(|e| {
            error!("Failed to get project root: {}", e);
            format!("{:?}", e)
        })?;

        info!("Project root detected: {:?}", root);

        let loader = Self {
            root: root.clone(),
            raw: root.join("raw"),
            from: root.join("from"),
        };

        Ok(loader)
    }

   pub fn load_raw(&self, filename: &str) -> io::Result<Vec<u8>> {
       let path = &self.raw.join(filename);
       std::fs::read(path)
   } 
}
