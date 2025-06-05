use std::fs::File;
use std::io::{BufReader, Read};

const CHUNK_SIZE: usize = 1024;

pub fn load_binary(path: &str) -> std::io::Result<()> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = [0u8; CHUNK_SIZE];
    println!("Loading...");
    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 { break; }
    }
    Ok(())
}
