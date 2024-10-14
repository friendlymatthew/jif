use std::path::PathBuf;

use eyre::Result;
use minifb::{Window, WindowOptions};

use clap::Parser;
use gif::{Decoder, dump_gif};
use gif::grammar::LogicalScreenDescriptor;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    gif_path: PathBuf,
}

fn main() -> Result<()> {
    let Args { gif_path } = Args::parse();

    let data = dump_gif(gif_path.to_str().expect("Failed to find path"))?;
    let mut decoder = Decoder::new(data);
    let compressed_gif = decoder.parse()?;

    let LogicalScreenDescriptor {
        canvas_width,
        canvas_height,
        ..
    } = compressed_gif.logical_screen_descriptor;

    let mut window = Window::new(
        "GIF renderer",
        canvas_width as usize,
        canvas_height as usize,
        WindowOptions::default(),
    )?;

    let frames = compressed_gif.decompress()?;

    let picture = &frames[0];

    while window.is_open() {
        window.update_with_buffer(picture, canvas_width as usize, canvas_height as usize)?;
    }

    Ok(())
}
