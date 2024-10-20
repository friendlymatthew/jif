use std::fmt::Debug;

use eyre::{eyre, Ok, OptionExt, Result};

use crate::bitstream::BitStream;
use crate::grammar::{
    ApplicationExtension, build_code_table, CommentExtension, DEFAULT_BACKGROUND_COLOR, DisposalMethod,
    Frame, GraphicControlExtension, ImageDescriptor, LogicalScreenDescriptor, parse_color_table,
    PlainTextExtension, TableBasedImage,
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

        let global_color_table = self
            .global_color_table
            .as_ref()
            .map(|t| parse_color_table(t.as_slice()));

        let background_color = match global_color_table.as_ref() {
            Some(gct) => *gct
                .get(background_color_index as usize)
                .ok_or_eyre("Background color is out of bounds")?,
            None => DEFAULT_BACKGROUND_COLOR,
        };

        let mut canvas = { vec![background_color; canvas_width as usize * canvas_height as usize] };

        let mut blocks_iter = self.blocks.iter();
        let mut frames = vec![];

        while let Some(mut block) = blocks_iter.next() {
            if block.special_purpose_block() {
                continue;
            }

            let graphic_control_extension = if let Block::GraphicControlExtension(gce) = block {
                block = blocks_iter
                    .next()
                    .ok_or_eyre("Expected graphic rending block.")?;

                Some(gce)
            } else {
                None
            };

            match block {
                Block::PlainTextExtension(_) => continue,
                Block::TableBasedImage(tbi) => {
                    let TableBasedImage {
                        image_descriptor,
                        image_data,
                        local_color_table,
                        lzw_minimum_code,
                    } = tbi;

                    let local_color_table = local_color_table
                        .as_ref()
                        .map(|t| parse_color_table(t.as_slice()));
                    let color_table = local_color_table
                        .as_ref()
                        .or(global_color_table.as_ref())
                        .ok_or_eyre("Failed to find color table.")?;

                    let mut code_table = build_code_table(color_table.len());

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
                            code_table = build_code_table(color_table.len());

                            let code = bitstream.next(current_code_len)?;
                            index_stream.extend(code_table[code].clone());
                            prev_code = code;
                            continue;
                        }

                        if next_code == eoi_code {
                            break;
                        }

                        if prev_code == usize::MAX {
                            return Err(eyre!("Expected initial code to be the clear code key. Got prev_code as usize::Max."));
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

                    let frame: Vec<u32> = index_stream
                        .iter()
                        .map(|index| color_table[*index])
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
                            Some(color_table[gce.transparent_color_index as usize])
                        } else {
                            None
                        }
                    });

                    let mut frame_coord = 0;

                    for row in image_top..image_top + image_height {
                        for i in 0..image_width {
                            let canvas_coord = (row as usize * canvas_width as usize)
                                + image_left as usize
                                + i as usize;

                            if frame_coord >= frame.len() {
                                return Err(eyre!("Improper slice into a Frame."));
                            }

                            if transparent_color.is_none()
                                || Some(frame[frame_coord]) != transparent_color
                            {
                                canvas[canvas_coord] = frame[frame_coord];
                            }

                            frame_coord += 1;
                        }
                    }

                    frames.push(Frame {
                        delay_time: graphic_control_extension.map(|gce| gce.delay_time),
                        pixels: canvas.clone(),
                    });

                    if let Some(gce) = graphic_control_extension {
                        match gce.disposal_method() {
                            DisposalMethod::NotRequired
                            | DisposalMethod::ToBeDefined
                            | DisposalMethod::DoNotDispose => {}
                            DisposalMethod::RestoreToBackground => {
                                canvas.fill(background_color);
                            }
                            DisposalMethod::RestoreToPrevious => {
                                let Frame { pixels, .. } = frames.last().ok_or_eyre(
                                    "Expected previous frame to exist, frames empty.",
                                )?;

                                canvas.copy_from_slice(pixels);
                            }
                        }
                    }
                }
                _ => return Err(eyre!("Encountered an out of order Block.")),
            }
        }

        Ok(frames)
    }
}
