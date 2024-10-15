use eyre::{Ok, Result};

#[derive(Debug)]
pub struct BitStream {
    // in bits
    pub cursor: usize,
    pub data: Vec<u8>,
}

impl BitStream {
    pub fn new(image_data: &Vec<Vec<u8>>) -> BitStream {
        let mut data = vec![];

        for image in image_data {
            data.extend(image);
        }

        BitStream { cursor: 0, data }
    }

    #[inline]
    pub fn eof(&self, bit_length: usize) -> bool {
        self.cursor + bit_length > self.data.len() * 8
    }

    #[inline]
    fn divmod_8(&self) -> (usize, usize) {
        (self.cursor / 8, self.cursor % 8)
    }

    pub fn read_bit(&mut self) -> u8 {
        let (byte_idx, bit_idx) = self.divmod_8();
        let byte = self.data[byte_idx];
        self.cursor += 1;
        (byte >> bit_idx) & 0b1
    }

    pub fn next(&mut self, bit_length: usize) -> Result<usize> {
        let bit_length = bit_length.min(12);

        let mut out = 0_usize;

        for i in 0..bit_length {
            out |= (self.read_bit() as usize) << i;
        }

        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_bit() -> Result<()> {
        let data = [
            0x8C, 0x2D, 0x99, 0x87, 0x2A, 0x1C, 0xDC, 0x33, 0xA0, 0x02, 0x75, 0xEC, 0x95, 0xFA,
            0xA8, 0xDE, 0x60, 0x8C, 0x04, 0x91, 0x4C, 0x01, 0x00,
        ];
        let mut bitstream = BitStream::new(&vec![data.to_vec()]);

        assert_eq!(bitstream.read_bit(), 0);
        assert_eq!(bitstream.read_bit(), 0);
        assert_eq!(bitstream.read_bit(), 1);
        assert_eq!(bitstream.read_bit(), 1);
        assert_eq!(bitstream.read_bit(), 0);
        assert_eq!(bitstream.read_bit(), 0);

        Ok(())
    }

    #[test]
    fn test_read_bytes() -> Result<()> {
        let data = [0x8c];

        let mut bitstream = BitStream::new(&vec![data.to_vec()]);

        assert_eq!(bitstream.next(3)?, 4);
        assert_eq!(bitstream.next(3)?, 1);

        Ok(())
    }

    #[test]
    fn test_read_dance_header() -> Result<()> {
        let data = [0, 157];

        let mut bitstream = BitStream::new(&vec![data.to_vec()]);

        assert_eq!(bitstream.read_bit(), 0);
        assert_eq!(bitstream.read_bit(), 0);
        assert_eq!(bitstream.read_bit(), 0);
        assert_eq!(bitstream.read_bit(), 0);
        assert_eq!(bitstream.read_bit(), 0);
        assert_eq!(bitstream.read_bit(), 0);
        assert_eq!(bitstream.read_bit(), 0);
        assert_eq!(bitstream.read_bit(), 0);
        assert_eq!(bitstream.read_bit(), 1);

        bitstream = BitStream::new(&vec![data.to_vec()]);
        assert_eq!(bitstream.next(9)?, 256);

        Ok(())
    }
}
