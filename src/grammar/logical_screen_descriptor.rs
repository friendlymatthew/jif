use std::fmt::Debug;

/// The LogicalScreenDescriptor contains the parameters necessary to define the
/// area of the display device within which the images will be rendered.
///
/// The coordinates in this block are given with respect to the top-left corner
/// of the virtual screen; they do not necessarily refer to absolute coordinates
/// on the display device. This implies that they could refer to window
/// coordinates in a window-based environment.
///
/// This block is REQUIRED; exactly one LogicalScreenDescriptor must be present.
#[derive(PartialEq)]
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
    pub fn global_color_table_flag(&self) -> bool {
        ((self.packed_field & 0b1000_0000) >> 7) == 1
    }

    /// Number of bits per primary color available to the original image, minus
    /// 1. This value represents the size of the entire palette from which the
    /// colors in the graphic were selected, not the number of colors actually
    /// used in the graphic.
    ///
    /// For example, if the value is 3, then the palette of the original image
    /// had 4 bits per primary color available to create the image.
    pub fn color_resolution(&self) -> u8 {
        ((self.packed_field & 0b0111_0000) >> 4) + 1
    }

    /// Indicates whether the Global Color Table is sorted. If the flag is set,
    /// the Global Color Table is sorted, in order of decreasing importance.
    /// Typically, the order would be decreasing frequency, with most frequency
    /// color first.
    pub fn sort_flag(&self) -> bool {
        ((self.packed_field & 0b1000) >> 3) == 1
    }

    /// If the [`global_color_table_flag`] is true, the value in this field is
    /// used to calculate the number of bytes contained in the Global Color
    /// Table.
    pub fn global_color_table_size(&self) -> usize {
        (1 << ((self.packed_field & 0b111) + 1)) * 3
    }
}

impl Debug for LogicalScreenDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LogicalScreenDescriptor {{
    canvas_width: {},
    canvas_height: {},
    global_color_table_flag: {},
    color_resolution: {},
    sort_flag: {},
    global_color_table_size: {}
    background_color_index: {},
    pixel_aspect_ratio: {},
}}",
            self.canvas_width,
            self.canvas_height,
            self.global_color_table_flag(),
            self.color_resolution(),
            self.sort_flag(),
            {
                if self.global_color_table_flag() {
                    self.global_color_table_size()
                } else {
                    0
                }
            },
            self.background_color_index,
            self.pixel_aspect_ratio,
        )
    }
}
