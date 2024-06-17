pub struct Byte {
    pub byte: u8,
}

impl Byte {
    pub fn nibbles(&self) -> [u8; 2] {
        [self.byte >> 4, self.byte & 0x0F]
    }
}
