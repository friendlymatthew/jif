use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;

use clap::Parser;
use eyre::Result;
use minifb::{Window, WindowOptions};

use jif::grammar::{Frame, LogicalScreenDescriptor};
use jif::{dump_gif, Decoder};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
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

    while window.is_open() {
        for frame in &frames {
            let Frame { pixels, delay_time } = frame;

            window.update_with_buffer(pixels, canvas_width as usize, canvas_height as usize)?;

            if let Some(delay_time) = delay_time {
                sleep(Duration::from_millis((delay_time * 10) as u64));
            }
        }
    }

    Ok(())
}
