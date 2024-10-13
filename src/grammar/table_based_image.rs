use std::fmt::Debug;

use super::ImageDescriptor;

pub struct TableBasedImage {
    pub image_descriptor: ImageDescriptor,
    pub local_color_table: Option<Vec<u8>>,
    pub lzw_minimum_code: u8,
    pub image_data: Vec<Vec<u8>>,
}

impl Debug for TableBasedImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TableBasedImage {{
    image_descriptor: {:?},
    local_color_table: {:?},
    lzw_minimum_code: {},
    image_data: {:?},
}}",
            self.image_descriptor,
            self.local_color_table,
            self.lzw_minimum_code,
            {
                let mut block_lens = vec![];

                for block in &self.image_data {
                    block_lens.push(block.len())
                }

                block_lens
            },
        )
    }
}
