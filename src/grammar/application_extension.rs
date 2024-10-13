#[derive(Debug)]
pub struct ApplicationExtension {
    pub identifier: String,
    pub authentication_code: [u8; 3],
    pub data: Vec<u8>,
}
