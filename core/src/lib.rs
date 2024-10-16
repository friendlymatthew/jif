use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use eyre::Result;

pub use decode::Decoder;

mod bitstream;
mod buffer;
mod decode;
pub mod gif_data_stream;
pub mod grammar;

pub fn dump_gif(path: &str) -> Result<Vec<u8>> {
    let path = PathBuf::from(path);
    let mut file = File::open(&path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}
