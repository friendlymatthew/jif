use std::fmt::Debug;

pub mod label {
    pub const EXTENSION: u8 = 0x21;
    pub const APPLICATION_EXTENSION: u8 = 0xFF;
    pub const COMMENT_EXTENSION: u8 = 0xFE;
    pub const GRAPHIC_CONTROL_EXTENSION: u8 = 0xF9;
    pub const IMAGE_DESCRIPTOR: u8 = 0x2C;
    pub const PLAIN_TEXT_EXTENSION: u8 = 0x01;
    pub const TRAILER: u8 = 0x3B;
}

#[derive(Debug)]
pub struct ApplicationExtension {
    pub identifier: String,
    pub authentication_code: [u8; 3],
    pub data: Vec<u8>,
}

pub type RGB = (u8, u8, u8);

pub fn build_code_table(size: usize) -> Vec<Vec<usize>> {
    (0..size + 2).map(|c| vec![c]).collect()
}

pub fn parse_color_table(color_table: &Vec<u8>) -> Vec<u32> {
    color_table
        .chunks_exact(3)
        .map(|chunk| {
            let (r, g, b) = (chunk[0], chunk[1], chunk[2]);
            u32::from_be_bytes([0u8, r, g, b])
        })
        .collect::<Vec<_>>()
}

#[derive(Debug)]
pub struct CommentExtension {
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub enum DisposalMethod {
    NotRequired,
    DoNotDispose,
    RestoreToBackground,
    RestoreToPrevious,
    ToBeDefined,
}

impl DisposalMethod {
    const fn from(d: u8) -> Self {
        match d {
            0 => Self::NotRequired,
            1 => Self::DoNotDispose,
            2 => Self::RestoreToBackground,
            3 => Self::RestoreToPrevious,
            _ => Self::ToBeDefined,
        }
    }
}

/// The GraphicControlExtension contains parameters used when processing a
/// graphic rendering block.
///
/// The scope of this extension is the first graphic rendering block to follow.
/// The extension contains only one data sub-block.
///
/// This block is OPTIONAL; at most one GraphicControlExtension may preced a
/// graphic rendering block.
#[derive(Debug, Clone)]
pub struct GraphicControlExtension {
    pub packed_field: u8,

    /// If not 0, this field specifies the number of hundredths (1/100) of a
    /// second to wait before continuing with the processing of the data stream.
    /// The clock starts ticking immediately after the graphic is rendered.
    pub delay_time: u16,

    /// The transparency index is such that when encountered, the corresponding
    /// pixel of the display device is not modified and processing goes onto the
    /// next pixel.
    pub transparent_color_index: u8,
}

impl GraphicControlExtension {
    /// Indicates the way in which the graphic is to be treated after being
    /// displayed.
    pub const fn disposal_method(&self) -> DisposalMethod {
        let disposal_method = (self.packed_field & 0b11100) >> 2;
        DisposalMethod::from(disposal_method)
    }

    /// Indicates whether user input is expected before continuing. If
    /// the flag is set, processing will continue when user input is entered.
    /// The nature of the User input is determined by the application (Carriage
    /// Return, Mouse Button Click, etc...).
    pub const fn user_input_flag(&self) -> bool {
        ((self.packed_field & 0b10) >> 1) == 1
    }

    /// Indicates whether a transparency index is given in the
    /// `transparent_color_index` field.
    pub const fn transparent_color_flag(&self) -> bool {
        (self.packed_field & 0b1) == 1
    }
}

/// The ImageDescriptor contains the parameters necessary to process a table
/// based image.
///
/// The coordinates in this block refer to coordinates within the Logical
/// Screen, and are given in pixels. The ImageDescriptor is always followed by
/// the image data.
///
/// This block is REQUIRED for an image. Exactly one ImageDescriptor must be
/// present per image in the data stream. An unlimited number of images may be
/// present per data stream.
#[derive(Debug, Clone)]
pub struct ImageDescriptor {
    /// Column number, in pixels, of the left edge of this image, with respect
    /// to the left edge of the Logical Screen. Leftmost column of the Logical
    /// Screen is 0.
    pub image_left: u16,

    /// Row number, in pixels, of the top edge of the image with respect to the
    /// top edge of the Logical Screen. Top row of the Logical Screen is 0.
    pub image_top: u16,

    /// Width of the image in pixels.
    pub image_width: u16,

    /// Height of the image in pixels.
    pub image_height: u16,

    pub packed_field: u8,
}

impl ImageDescriptor {
    /// Indicates the presence of a LocalColorTable immediately following this
    /// ImageDescriptor.
    pub const fn local_color_table_flag(&self) -> bool {
        ((self.packed_field & 0b1000_0000) >> 7) == 1
    }

    /// Indicates if the image is interlaced. An image is interlaced in a
    /// four-pass interlace pattern.
    pub const fn interlace_flag(&self) -> bool {
        ((self.packed_field & 0b100_0000) >> 6) == 1
    }

    /// Indicates whether the LocalColorTable is sorted. If the flag is set, the
    /// LocalColorTable is sorted, in order of decreasing importance. Typically,
    /// the order would be decreasing frequency, with most frequent color first.
    pub const fn sort_flag(&self) -> bool {
        ((self.packed_field & 0b10_0000) >> 4) == 1
    }

    /// If the [`local_color_table_flag`] is true, the value in this field is
    /// used to calculate the number of bytes contained in the Local Color
    /// Table.
    pub const fn local_color_table_size(&self) -> usize {
        3 * (1 << ((self.packed_field & 0b111) + 1))
    }
}

pub const DEFAULT_BACKGROUND_COLOR: u32 = 0_u32;

/// The LogicalScreenDescriptor contains the parameters necessary to define the
/// area of the display device within which the images will be rendered.
///
/// The coordinates in this block are given with respect to the top-left corner
/// of the virtual screen; they do not necessarily refer to absolute coordinates
/// on the display device. This implies that they could refer to window
/// coordinates in a window-based environment.
///
/// This block is REQUIRED; exactly one LogicalScreenDescriptor must be present.
#[derive(Debug, PartialEq, Eq)]
pub struct LogicalScreenDescriptor {
    /// Width, in pixels, of the Logical Screen where images will be rendered.
    pub canvas_width: u16,

    /// Height, in pixels, of the Logical Screen where images will be rendered.
    pub canvas_height: u16,

    pub packed_field: u8,

    /// Index into the global color table for background color. The background
    /// color is the color used for those pixels on the screen that are not
    /// covered by an image. If `global_color_table_size` is 0, ignore this
    /// field.
    pub background_color_index: u8,

    /// Factor used to compute an approximation of the aspect ratio of the
    /// pixel in the origin.
    pub pixel_aspect_ratio: u8,
}

impl LogicalScreenDescriptor {
    /// Flag indicating the prescence of a Global Color Table.
    pub const fn global_color_table_flag(&self) -> bool {
        ((self.packed_field & 0b1000_0000) >> 7) == 1
    }

    /// Number of bits per primary color available to the original image - 1.
    ///
    /// This value represents the size of the entire palette from which the
    /// colors in the graphic were selected, not the number of colors
    /// actually used in the graphic.
    ///
    /// For example, if the value is 3, then the palette of the original image
    /// had 4 bits per primary color available to create the image.
    pub const fn color_resolution(&self) -> u8 {
        ((self.packed_field & 0b0111_0000) >> 4) + 1
    }

    /// Indicates whether the Global Color Table is sorted. If the flag is set,
    /// the Global Color Table is sorted, in order of decreasing importance.
    /// Typically, the order would be decreasing frequency, with most frequency
    /// color first.
    pub const fn sort_flag(&self) -> bool {
        ((self.packed_field & 0b1000) >> 3) == 1
    }

    /// If the [`global_color_table_flag`] is true, the value in this field is
    /// used to calculate the number of bytes contained in the Global Color
    /// Table.
    pub const fn global_color_table_size(&self) -> usize {
        (1 << ((self.packed_field & 0b111) + 1)) * 3
    }
}

/// The Plain Text Extension contains textual data and the parameters necessary
/// to render that data as a graphic, in a simple form.
///
/// The textual data will be encoded with the 7-bit printable ASCII characters.
/// Text data are rendered using a grid of character cells defined by the
/// parameters in the block fields. Each character is rendered in an individual
/// cell. The textual data in this block is to be rendered as mono-spaced
/// characters, one character per cell, with a best fitting font and size. The
/// data characters are taken sequentially from the data portion of the block
/// and rendered within a cell, starting with the upper left cell in the grid
/// and proceeding from left to right and from top to bottom. Text data is
/// rendered until the end of data is reached or the character grid is filled.
/// The Character Grid contains an integral number of cells; in the case that
/// the cell dimensions do not allow for an integral number, fractional cells
/// must be discarded; an encoder must be careful to specify the grid dimensions
/// accurately so that this does not happen. This block requires a Global Color
/// Table to be available; the colors used by this block reference the Global
/// Color Table in the Stream if there is one, or the Global Color Table from a
/// previous Stream, if one was saved. This block is a graphic rendering block,
/// therefore it may be modified by a Graphic Control Extension.
#[derive(Debug)]
pub struct PlainTextExtension {
    /// Column number, in pixels, of the left edge of the text grid, with
    /// respect to the left edge of the Logical Screen.
    pub text_grid_left_position: u16,

    /// Row number, in pixels, of the top edge of the text grid, with respect to
    /// the top edge of the Logical Screen.
    pub text_grid_top_position: u16,

    /// Width of the text grid in pixels.
    pub text_grid_width: u16,

    /// Height of the text grid in pixels.
    pub text_grid_height: u16,

    /// Width, in pixels, of each cell in the grid.
    pub character_cell_width: u8,

    /// Height, in pixels, of each cell in the grid.
    pub character_cell_height: u8,

    /// Index into the Global Color Table to be used to render the text
    /// foreground.
    pub text_foreground_color_index: u8,

    /// Index into the Global Color Table to be used to render the text
    /// background.
    pub text_background_color_index: u8,

    /// Sequence of sub-blocks, each of size at most 255 bytes and at least 1
    /// byte, with the size in a byte preceding the data.
    pub plain_text_data: Vec<u8>,
}

#[derive(Debug)]
pub struct TableBasedImage {
    pub image_descriptor: ImageDescriptor,
    pub local_color_table: Option<Vec<u8>>,
    pub lzw_minimum_code: u8,
    pub image_data: Vec<Vec<u8>>,
}

#[derive(Debug)]
pub struct Frame {
    pub delay_time: Option<u16>,
    pub pixels: Vec<u32>,
}
