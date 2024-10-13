use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use eyre::Result;

pub use decode::Decoder;

mod buffer;
mod decode;
pub mod grammar;

pub fn dump_gif(path: &str) -> Result<Vec<u8>> {
    let path = PathBuf::from(path);
    let mut file = File::open(&path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}
