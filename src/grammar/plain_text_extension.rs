/// The Plain Text Extension contains textual data and the parameters necessary
/// to render that data as a graphic, in a simple form. The textual data will be
/// encoded with the 7-bit printable ASCII characters. Text data are rendered
/// using a grid of character cells defined by the parameters in the block
/// fields. Each character is rendered in an individual cell. The textual data
/// in this block is to be rendered as mono-spaced characters, one character per
/// cell, with a best fitting font and size. The data characters are taken
/// sequentially from the data portion of the block and rendered within a cell,
/// starting with the upper left cell in the grid and proceeding from left to
/// right and from top to bottom. Text data is rendered until the end of data is
/// reached or the character grid is filled. The Character Grid contains an
/// integral number of cells; in the case that the cell dimensions do not allow
/// for an integral number, fractional cells must be discarded; an encoder must
/// be careful to specify the grid dimensions accurately so that this does not
/// happen. This block requires a Global Color Table to be available; the colors
/// used by this block reference the Global Color Table in the Stream if there
/// is one, or the Global Color Table from a previous Stream, if one was saved.
/// This block is a graphic rendering block, therefore it may be modified by a
/// Graphic Control Extension.
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
