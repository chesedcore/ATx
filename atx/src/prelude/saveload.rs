//loader.rs

use log::{info, error};
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Debug)]
pub struct Saveloader {
    pub root: PathBuf,
    pub raw: PathBuf,
    pub from: PathBuf,
}

impl Saveloader {
    
    fn find_dev_root() -> Option<PathBuf> {
        let mut current = std::env::current_exe().ok()?;

        // Walk up looking for Cargo.toml
        while current.pop() {
            if current.join("Cargo.toml").exists() {
                return Some(current);
            }
        }

        None
    }

    pub fn new() -> io::Result<Self> {
        let root = Self::find_dev_root().or_else(|| {
            std::env::current_exe().ok()
                .and_then(|p| p.parent().map(|p| p.to_path_buf()))

        }).ok_or_else(|| {
            io::Error::new(io::ErrorKind::Other, "Failed to determine executable or project root.")
        })?;

        info!("ATx root directory: {:?}", root);

        Ok(Self {
            root: root.clone(),
            raw: root.join("raw"),
            from: root.join("from"),
        })
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
