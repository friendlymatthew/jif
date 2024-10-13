mod application_extension;
mod bitstream;
mod color_table;
mod comment_extension;
mod gif;
mod graphic_control_extension;
mod image_descriptor;
mod logical_screen_descriptor;
mod plain_text_extension;
mod table_based_image;

pub mod label {
    pub const EXTENSION: u8 = 0x21;
    pub const APPLICATION_EXTENSION: u8 = 0xFF;
    pub const COMMENT_EXTENSION: u8 = 0xFE;
    pub const GRAPHIC_CONTROL_EXTENSION: u8 = 0xF9;
    pub const IMAGE_DESCRIPTOR: u8 = 0x2C;
    pub const PLAIN_TEXT_EXTENSION: u8 = 0x01;
    pub const TRAILER: u8 = 0x3B;
}

pub use application_extension::*;
pub use bitstream::*;
pub use color_table::*;
pub use comment_extension::*;
pub use gif::*;
pub use graphic_control_extension::*;
pub use image_descriptor::*;
pub use logical_screen_descriptor::*;
pub use plain_text_extension::*;
pub use table_based_image::*;
