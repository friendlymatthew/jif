use eyre::{eyre, Ok, Result};

use crate::grammar::label::TRAILER;

#[derive(Debug)]
pub struct Buffer {
    cursor: usize,
    data: Vec<u8>,
}

impl Buffer {
    pub const fn new(data: Vec<u8>) -> Self {
        Self { cursor: 0, data }
    }

    pub const fn _current(&self) -> usize {
        self.cursor
    }

    pub fn next(&mut self) -> Result<u8> {
        self.eof(0)?;

        let b = self.data[self.cursor];
        self.cursor += 1;

        Ok(b)
    }

    pub fn _next_by(&mut self, bytes: usize) -> Result<&[u8]> {
        self.eof(bytes - 1)?;

        let slice = &self.data[self.cursor..self.cursor + bytes];
        self.cursor += bytes;

        Ok(slice)
    }

    pub fn expect<const N: usize>(&mut self, bytes: [u8; N]) -> Result<()> {
        self.eof(N - 1)?;

        if bytes != self.data[self.cursor..self.cursor + N] {
            return Err(eyre!(
                "Unexpected slice {:?}, expected: {:?}",
                &bytes,
                &self.data[self.cursor..self.cursor + N]
            ));
        }

        self.cursor += N;

        Ok(())
    }

    // raises an error if out of bounds
    fn eof(&self, bytes: usize) -> Result<()> {
        if self.cursor + bytes < self.data.len() {
            return Ok(());
        }

        Err(eyre!("Unexpected EOF"))
    }

    pub fn read_u16(&mut self) -> Result<u16> {
        self.eof(1)?;

        let b = u16::from_le_bytes([self.data[self.cursor], self.data[self.cursor + 1]]);
        self.cursor += 2;

        Ok(b)
    }

    pub fn read_slice(&mut self, bytes: usize) -> Result<Vec<u8>> {
        self.eof(bytes - 1)?;
        let slice = self.data[self.cursor..self.cursor + bytes].to_owned();
        self.cursor += bytes;
        Ok(slice)
    }

    pub fn at_end(&self) -> bool {
        self.cursor == self.data.len()
            || (self.cursor == self.data.len() && self.data[self.cursor] != TRAILER)
    }
}
