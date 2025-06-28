//loader.rs

use log::{info, error};
use std::io::{self, Write};
use std::path::PathBuf;
use project_root::get_project_root;

#[derive(Debug)]
pub struct Saveloader {
    pub root: PathBuf,
    pub raw: PathBuf,
    pub from: PathBuf,
}

impl Saveloader {
    pub fn new() -> io::Result<Self> {
        let root = get_project_root().map_err(|e| {
            error!("Failed to get project root: {}", e);
            std::io::Error::new(std::io::ErrorKind::UnexpectedEof, format!("{:?}", e))
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
       let path = self.raw.join(filename);
       std::fs::read(path)
   }

   pub fn save_from(&self, filename: &str, data: Vec<String>) -> io::Result<()> {
       let mut file = std::fs::OpenOptions::new()
                                        .write(true)
                                        .create(true)
                                        .truncate(true)
                                        .open(self.from.join(filename))?;

       let content = data.join("\n");
       file.write_all(content.as_bytes())?;
       Ok(())
   }
}
