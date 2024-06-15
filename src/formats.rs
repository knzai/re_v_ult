pub struct Byte {
    pub byte: u8,
}

impl Byte {
    pub fn nibbles(&self) -> [u8; 2] {
        [self.byte >> 4, self.byte & 0x0F]
    }

    pub fn couplets(&self) -> [u8; 4] {
        [
            self.byte >> 6,
            self.byte & 0b00110000 >> 4,
            self.byte & 0b00001100 >> 2,
            self.byte & 0b00000011,
        ]
    }

    // pub fn r_couplets(&self) -> [u8; 4] {
    //     [
    //         self.byte & 0b00000011,
    //         self.byte & 0b00001100 >> 2,
    //         self.byte & 0b00110000 >> 4,
    //         self.byte >> 6,
    //     ]
    // }
}
