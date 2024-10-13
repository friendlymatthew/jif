use std::fmt::Debug;

/// The ImageDescriptor contains the parameters necessary to process a table
/// based image. The coordinates in this block refer to coordinates within the
/// Logical Screen, and are given in pixels. The ImageDescriptor is always
/// followed by the image data.
///
/// This block is REQUIRED for an image. Exactly one ImageDescriptor must be
/// present per image in the data stream. An unlimited number of images may be
/// present per data stream.
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
    pub fn local_color_table_flag(&self) -> bool {
        ((self.packed_field & 0b1000_0000) >> 7) == 1
    }

    /// Indicates if the image is interlaced. An image is interlaced in a
    /// four-pass interlace pattern.
    pub fn interlace_flag(&self) -> bool {
        ((self.packed_field & 0b100_0000) >> 6) == 1
    }

    /// Indicates whether the LocalColorTable is sorted. If the flag is set, the
    /// LocalColorTable is sorted, in order of decreasing importance. Typically,
    /// the order would be decreasing frequency, with most frequent color first.
    pub fn sort_flag(&self) -> bool {
        ((self.packed_field & 0b10_0000) >> 4) == 1
    }

    /// If the [`local_color_table_flag`] is true, the value in this field is
    /// used to calculate the number of bytes contained in the Local Color
    /// Table.
    pub fn local_color_table_size(&self) -> usize {
        3 * (1 << ((self.packed_field & 0b111) + 1))
    }
}

impl Debug for ImageDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ImageDescriptor {{
    image_left: {},
    image_top: {},
    image_width: {},
    image_height: {},
    local_color_table_flag: {},
    interlace_flag: {},
    sort_flag: {},
    local_color_table_size: {},
}}",
            self.image_left,
            self.image_top,
            self.image_width,
            self.image_height,
            self.local_color_table_flag(),
            self.interlace_flag(),
            self.sort_flag(),
            {
                if self.local_color_table_flag() {
                    self.local_color_table_size()
                } else {
                    0
                }
            }
        )
    }
}
