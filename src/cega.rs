pub mod binary {
    // use funty::Unsigned;
    //
    //     pub fn to_bits<T: Unsigned>(byte: T, len: u8) -> Vec<u8> {
    //         (0..8 / len).rev().map(|n| (byte >> n) & 1).collect()
    //     }

    pub fn to_bits(byte: u8, len: u8) -> Vec<u8> {
        (0..8 / len).rev().map(|n| (byte >> n) & 1).collect()
    }

    #[test]
    fn to_bits_handles_u8() {
        let byte: u8 = 0b00011011;
        assert_eq!(to_bits(byte, 1), [0, 0, 0, 1, 1, 0, 1, 1]);
    }
}
