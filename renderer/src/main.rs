use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;

use clap::Parser;
use eyre::Result;
use minifb::{Window, WindowOptions};

use jif::{Decoder, dump_gif};
use jif::grammar::{Frame, LogicalScreenDescriptor};

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

    window.set_target_fps(30);

    let frames = compressed_gif.decompress()?;

    while window.is_open() {
        for frame in &frames {
            let Frame {
                image_descriptor,
                pixels,
                graphic_control_extension,
            } = frame;

            window.update_with_buffer(
                pixels,
                image_descriptor.image_width as usize,
                image_descriptor.image_height as usize,
            )?;

            if let Some(gce) = graphic_control_extension {
                sleep(Duration::from_millis((gce.delay_time * 10) as u64));
            }
        }
    }

    Ok(())
}
