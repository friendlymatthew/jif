#[macro_use]
extern crate afl;
extern crate core;

use jif::Decoder;

fn main() {
    fuzz!(|data: &[u8]| {
        let mut decoder = Decoder::new(data.to_vec());
        let _ = decoder.decode();
    })
}
