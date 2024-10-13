use std::fmt::Debug;

#[derive(Debug)]
pub enum DisposalMethod {
    NotRequired,
    DoNotDispose,
    RestoreToBackground,
    RestoreToPrevious,
    ToBeDefined,
}

impl DisposalMethod {
    fn from(d: u8) -> DisposalMethod {
        match d {
            0 => DisposalMethod::NotRequired,
            1 => DisposalMethod::DoNotDispose,
            2 => DisposalMethod::RestoreToBackground,
            3 => DisposalMethod::RestoreToPrevious,
            _ => DisposalMethod::ToBeDefined,
        }
    }
}

/// The GraphicControlExtension contains parameters used when processing a
/// graphic rendering block. The scope of this extension is the first graphic
/// rendering block to follow. The extension contains only one data sub-block.
///
/// This block is OPTIONAL; at most one GraphicControlExtension may preced a
/// graphic rendering block.
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
    pub fn disposal_method(&self) -> DisposalMethod {
        let disposal_method = (self.packed_field & 0b11100) >> 2;
        DisposalMethod::from(disposal_method)
    }

    /// Indicates whether or not user input is expected before continuing. If
    /// the flag is set, processing will continue when user input is entered.
    /// The nature of the User input is determined by the application (Carriage
    /// Return, Mouse Button Click, etc...).
    pub fn user_input_flag(&self) -> bool {
        ((self.packed_field & 0b10) >> 1) == 1
    }

    /// Indicates whether a transparency index is given in the
    /// `transparent_color_index` field.
    pub fn transparent_color_flag(&self) -> bool {
        (self.packed_field & 0b1) == 1
    }
}

impl Debug for GraphicControlExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "GraphicControlExtension {{
    disposal_method: {:?},
    user_input_flag: {},
    transparent_color_flag: {},
    delay_time: {},
    transparent_color_index: {},
}}",
            self.disposal_method(),
            self.user_input_flag(),
            self.transparent_color_flag(),
            self.delay_time,
            self.transparent_color_index
        )
    }
}
