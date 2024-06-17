pub mod binary {
    use funty::Unsigned;

    pub fn bit_vec<T: Unsigned>(byte: T, len: u8) -> Vec<u8>
    where
        u8: From<T>,
    {
        let mask: u8 = 2_u8.pow(len.into()) - 1;
        match len {
            1 | 2 | 4 => (0..8 / len)
                .rev()
                .map(|n| {
                    let shift: u8 = n * len;
                    let shifted: u8 = (byte >> shift).into();
                    shifted & mask
                })
                .collect(),
            _ => panic!("invalid word length"),
        }
    }

    #[test]
    fn bit_vec_handles_u8() {
        let byte: u8 = 0b00011011;
        assert_eq!(bit_vec(byte, 1), [0, 0, 0, 1, 1, 0, 1, 1]);
        assert_eq!(bit_vec(byte, 2), [0b00, 0b01, 0b10, 0b11]);
        assert_eq!(bit_vec(byte, 4), [0b0001, 0b1011]);
    }
}
