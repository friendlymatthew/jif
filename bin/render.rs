use eyre::Result;
use minifb::{Window, WindowOptions};

use gif::{Decoder, dump_gif};
use gif::grammar::LogicalScreenDescriptor;

fn main() -> Result<()> {
    let data = dump_gif("/Users/matthew/Desktop/gif/sample_1.gif")?;
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
