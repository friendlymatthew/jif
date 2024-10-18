use eyre::{eyre, Result};

use crate::{
    buffer::Buffer,
    grammar::{
        ApplicationExtension,
        CommentExtension, GraphicControlExtension, ImageDescriptor, label::{
            APPLICATION_EXTENSION, COMMENT_EXTENSION, EXTENSION, GRAPHIC_CONTROL_EXTENSION,
            IMAGE_DESCRIPTOR, PLAIN_TEXT_EXTENSION,
        },
        LogicalScreenDescriptor, PlainTextExtension, TableBasedImage,
    },
};
use crate::gif_data_stream::{Block, GifDataStream};

/// The decoder is the program used to process a GIF data stream. It processes
/// the data stream sequentially, parsing the various blocks and sub-blocks,
/// using control information to set hardware and process parameters and
/// interpreting the data to render the graphics.
#[derive(Debug)]
pub struct Decoder {
    buffer: Buffer,
}

impl Decoder {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            buffer: Buffer::new(data),
        }
    }

    pub fn parse(&mut self) -> Result<GifDataStream> {
        let buffer = &mut self.buffer;
        buffer.expect(*b"GIF")?;
        let version = String::from_utf8(buffer.read_slice(3)?)?;

        // logical_screen_descriptor
        let logical_screen_descriptor = LogicalScreenDescriptor {
            canvas_width: buffer.read_u16(),
            canvas_height: buffer.read_u16(),
            packed_field: buffer.next(),
            background_color_index: buffer.next(),
            pixel_aspect_ratio: buffer.next(),
        };

        let global_color_table = if logical_screen_descriptor.global_color_table_flag() {
            let global_color_table_size = logical_screen_descriptor.global_color_table_size();

            let buffer = buffer.read_slice(global_color_table_size)?;

            Some(buffer)
        } else {
            None
        };

        let mut blocks = vec![];

        // this loop iterates by every <Data> block
        while !buffer.at_end() {
            let byte = buffer.next();

            if byte == EXTENSION {
                match buffer.next() {
                    APPLICATION_EXTENSION => {
                        let _block_size = buffer.next() as usize;
                        let application_extension = ApplicationExtension {
                            identifier: String::from_utf8(buffer.read_slice(8)?)?,
                            authentication_code: [buffer.next(), buffer.next(), buffer.next()],
                            data: {
                                let data_size = buffer.next() as usize;
                                buffer.read_slice(data_size)?
                            },
                        };

                        buffer.next();
                        blocks.push(Block::ApplicationExtension(application_extension));
                    }
                    COMMENT_EXTENSION => {
                        let block_size = buffer.next();

                        let comment_extension = CommentExtension {
                            data: buffer.read_slice(block_size as usize)?,
                        };

                        let _term_byte = buffer.next();

                        blocks.push(Block::CommentExtension(comment_extension));
                    }
                    GRAPHIC_CONTROL_EXTENSION => {
                        let _block_size = buffer.next();
                        let graphic_control_extension = GraphicControlExtension {
                            packed_field: buffer.next(),
                            delay_time: buffer.read_u16(),
                            transparent_color_index: buffer.next(),
                        };

                        let _term_byte = buffer.next();

                        blocks.push(Block::GraphicControlExtension(graphic_control_extension));
                    }
                    PLAIN_TEXT_EXTENSION => {
                        if global_color_table.as_ref().is_none() {
                            return Err(eyre!(
                                "This block requires a Global Color Table to be available."
                            ));
                        }

                        let block_size = buffer.next();

                        let plain_text_extension = PlainTextExtension {
                            text_grid_left_position: buffer.read_u16(),
                            text_grid_top_position: buffer.read_u16(),
                            text_grid_width: buffer.read_u16(),
                            text_grid_height: buffer.read_u16(),
                            character_cell_width: buffer.next(),
                            character_cell_height: buffer.next(),
                            text_foreground_color_index: buffer.next(),
                            text_background_color_index: buffer.next(),
                            plain_text_data: buffer.read_slice(block_size as usize - 12)?,
                        };

                        let _term_byte = buffer.next();
                        blocks.push(Block::PlainTextExtension(plain_text_extension));
                    }
                    _ => return Err(eyre!("Encountered an inner block extension")),
                }
            } else if byte == IMAGE_DESCRIPTOR {
                let image_descriptor = ImageDescriptor {
                    image_left: buffer.read_u16(),
                    image_top: buffer.read_u16(),
                    image_width: buffer.read_u16(),
                    image_height: buffer.read_u16(),
                    packed_field: buffer.next(),
                };

                let local_color_table = if image_descriptor.local_color_table_flag() {
                    let local_color_table_size = image_descriptor.local_color_table_size();

                    Some(buffer.read_slice(local_color_table_size)?)
                } else {
                    None
                };

                let lzw_minimum_code = buffer.next();

                let mut sub_blocks = vec![];

                let mut block_size = buffer.next();

                while block_size != 0 {
                    sub_blocks.push(buffer.read_slice(block_size as usize)?);
                    block_size = buffer.next();
                }

                blocks.push(Block::TableBasedImage(TableBasedImage {
                    image_descriptor,
                    local_color_table,
                    lzw_minimum_code,
                    image_data: sub_blocks,
                }));
            }
        }

        Ok(GifDataStream {
            version,
            logical_screen_descriptor,
            global_color_table,
            blocks,
        })
    }

    pub fn decode(&mut self) -> Result<()> {
        let gif = self.parse()?;
        gif.decompress()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::dump_gif;

    use super::*;

    #[test]
    fn parse() -> Result<()> {
        let data = dump_gif("../jersey-dance.gif")?;

        let mut decoder = Decoder::new(data);
        let compressed_gif = decoder.parse()?;

        let first_image = compressed_gif
            .blocks
            .iter()
            .find(|block| matches!(block, Block::TableBasedImage(_)))
            .unwrap();

        if let Block::TableBasedImage(tbi) = first_image {
            assert_eq!([0, 157], tbi.image_data[0][0..2]);
        }

        Ok(())
    }

    #[test]
    fn decode() -> Result<()> {
        let data = dump_gif("../sample_1.gif")?;

        let mut decoder = Decoder::new(data);
        let compressed_gif = decoder.parse()?;

        let frames = compressed_gif.decompress()?;
        let (_, init_frame) = &frames[0];

        assert_eq!(init_frame.len(), 100);

        Ok(())
    }
}
