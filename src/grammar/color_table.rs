pub type RGB = (u8, u8, u8);

pub fn build_code_table(size: usize) -> Vec<Vec<usize>> {
    (0..size / 3 + 2).map(|c| vec![c]).collect()
}
