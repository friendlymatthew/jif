use std::fmt::Debug;

use eyre::{eyre, Ok, OptionExt, Result};

use crate::bitstream::BitStream;
use crate::grammar::{
    ApplicationExtension, build_code_table, CommentExtension, DisposalMethod, Frame,
    GraphicControlExtension, ImageDescriptor, LogicalScreenDescriptor, PlainTextExtension,
    TableBasedImage,
};

#[derive(Debug)]
pub enum Block {
    GraphicControlExtension(GraphicControlExtension),
    TableBasedImage(TableBasedImage),
    PlainTextExtension(PlainTextExtension),
    ApplicationExtension(ApplicationExtension),
    CommentExtension(CommentExtension),
}

impl Block {
    const fn special_purpose_block(&self) -> bool {
        matches!(
            self,
            Self::ApplicationExtension(_) | Self::CommentExtension(_)
        )
    }
}

#[derive(Debug)]
pub struct GifDataStream {
    pub version: String,
    pub logical_screen_descriptor: LogicalScreenDescriptor,
    pub global_color_table: Option<Vec<u8>>,
    pub blocks: Vec<Block>,
}

impl GifDataStream {
    pub fn decompress(&self) -> Result<Vec<Frame>> {
        let LogicalScreenDescriptor {
            canvas_width,
            canvas_height,
            background_color_index,
            ..
        } = self.logical_screen_descriptor;

        let global_pixels_table = if let Some(gct) = &self.global_color_table {
            Some(
                gct.chunks_exact(3)
                    .map(|chunk| {
                        let (r, g, b) = (chunk[0], chunk[1], chunk[2]);
                        u32::from_be_bytes([0u8, r, g, b])
                    })
                    .collect::<Vec<_>>(),
            )
        } else {
            None
        };

        let background_color = global_pixels_table.as_ref().map_or(Ok(0u32), |gpt| {
            let background_color_index = background_color_index as usize;
            if gpt.len() <= background_color_index {
                return Err(eyre!("Background color index is out of bounds."));
            }

            Ok(gpt[background_color_index])
        })?;

        let mut pixel_buffer =
            { vec![background_color; canvas_width as usize * canvas_height as usize] };

        let mut blocks_iter = self.blocks.iter();
        let mut frames = vec![];

        while let Some(block) = blocks_iter.next() {
            if block.special_purpose_block() {
                continue;
            }

            let graphic_control_extension = if let Block::GraphicControlExtension(gce) = block {
                Some(gce)
            } else {
                None
            };

            match blocks_iter.next() {
                Some(Block::PlainTextExtension(_)) => {}
                Some(Block::TableBasedImage(tbi)) => {
                    let TableBasedImage {
                        image_descriptor,
                        image_data,
                        local_color_table,
                        lzw_minimum_code,
                    } = tbi;

                    let initial_code_table_len = if let Some(lct) = local_color_table {
                        lct.len()
                    } else if let Some(gct) = &self.global_color_table {
                        gct.len()
                    } else {
                        return Err(eyre!(
                            "Failed to find color table. No global or local color table found."
                        ));
                    };

                    let mut code_table = build_code_table(initial_code_table_len);

                    let clear_code_key = 2_usize.pow(*lzw_minimum_code as u32);
                    let eoi_code = clear_code_key + 1;

                    let mut bitstream = BitStream::new(image_data);
                    let mut current_code_len = *lzw_minimum_code as usize + 1;
                    let mut index_stream = vec![];
                    let mut prev_code = usize::MAX;

                    while !bitstream.eof(current_code_len) {
                        let next_code = bitstream.next(current_code_len)?;

                        if next_code == clear_code_key {
                            current_code_len = (*lzw_minimum_code + 1) as usize;
                            code_table = build_code_table(initial_code_table_len);

                            let code = bitstream.next(current_code_len)?;
                            index_stream.extend(code_table[code].clone());
                            prev_code = code;
                            continue;
                        }

                        if next_code == eoi_code {
                            break;
                        }

                        if prev_code == usize::MAX || prev_code >= code_table.len() {
                            return Err(eyre!("Expected initial code to be the clear code key."));
                        }

                        if next_code < code_table.len() {
                            let colors = &code_table[next_code];
                            index_stream.extend(colors.clone());

                            let k = colors.first().ok_or_eyre("Failed to get any color")?;

                            let mut new_colors = code_table[prev_code].clone();
                            new_colors.push(*k);
                            code_table.push(new_colors);
                        } else {
                            let colors = &code_table[prev_code];
                            let k = colors.first().ok_or_eyre("Failed to get color")?;

                            let mut new_sequence = colors.clone();
                            new_sequence.push(*k);

                            index_stream.extend(new_sequence.clone());
                            code_table.push(new_sequence.clone());
                        }

                        prev_code = next_code;

                        if code_table.len() == 2usize.pow(current_code_len as u32) {
                            current_code_len += 1;
                        }
                    }

                    let pixels_table = if let Some(lct) = local_color_table {
                        &lct.chunks_exact(3)
                            .map(|chunk| {
                                let [r, g, b] = chunk else {
                                    return Err(eyre!("Chunk is not a multiple of 3."));
                                };

                                Ok(u32::from_be_bytes([0u8, *r, *g, *b]))
                            })
                            .collect::<Result<Vec<_>>>()?
                    } else if let Some(gpt) = &global_pixels_table {
                        gpt
                    } else {
                        return Err(eyre!(
                            "Failed to find color table. No global or color table found."
                        ));
                    };

                    let pixels: Vec<u32> = index_stream
                        .iter()
                        .map(|index| pixels_table[*index])
                        .collect();

                    let &ImageDescriptor {
                        image_left,
                        image_top,
                        image_width,
                        image_height,
                        ..
                    } = image_descriptor;

                    let transparent_color = graphic_control_extension.as_ref().and_then(|gce| {
                        if gce.transparent_color_flag() {
                            Some(pixels_table[gce.transparent_color_index as usize])
                        } else {
                            None
                        }
                    });

                    let mut frame_coord = 0;

                    for row in image_top..image_top + image_height {
                        for i in 0..image_width {
                            let buffer_coord = (row as usize * canvas_width as usize)
                                + image_left as usize
                                + i as usize;

                            if transparent_color.is_none()
                                || Some(pixels[frame_coord]) != transparent_color
                            {
                                pixel_buffer[buffer_coord] = pixels[frame_coord];
                            }

                            frame_coord += 1;
                        }
                    }

                    if let Some(gce) = graphic_control_extension {
                        match gce.disposal_method() {
                            DisposalMethod::NotRequired
                            | DisposalMethod::ToBeDefined
                            | DisposalMethod::DoNotDispose => {}
                            DisposalMethod::RestoreToBackground => {
                                todo!();
                            }
                            DisposalMethod::RestoreToPrevious => {
                                todo!();
                            }
                        }
                    }

                    frames.push(Frame {
                        delay_time: {
                            if let Some(gce) = graphic_control_extension {
                                Some(gce.delay_time)
                            } else {
                                None
                            }
                        },
                        pixels: pixel_buffer.clone(),
                    });
                }
                Some(_) => return Err(eyre!("Encountered an out of order Block.")),
                None => return Err(eyre!("Unexpected EOF.")),
            }
        }

        Ok(frames)
    }
}
