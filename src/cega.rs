pub mod binary {
    use bitvec::prelude::*;

    #[test]
    fn test_bitvec() {
        let byte8: u8 = 0b00011011;
        let mut chunks = byte8.view_bits::<Msb0>().chunks(2);
        assert_eq!(chunks.next().unwrap().load::<u8>(), 0);
        assert_eq!(chunks.next().unwrap().load::<u8>(), 1);
        assert_eq!(chunks.next().unwrap().load::<u8>(), 2);
        assert_eq!(chunks.next().unwrap().load::<u8>(), 3);
    }
    // fn bit_vec_handles_u8() {
    //     let byte8: u8 = 0b00011011;
    //     //let byte16: u16 = 0b0001101110110001;
    //     assert_eq!(bit_vec(byte8, 1), [0, 0, 0, 1, 1, 0, 1, 1]);
    //     assert_eq!(bit_vec(byte8, 2), [0b00, 0b01, 0b10, 0b11]);
    //     assert_eq!(bit_vec(byte8, 4), [0b0001, 0b1011]);
    //     // assert_eq!(bit_vec(byte16, 1), [0, 0, 0, 1, 1, 0, 1, 1]);
    //     // assert_eq!(bit_vec(byte16, 2), [0b00, 0b01, 0b10, 0b11]);
    //     // assert_eq!(bit_vec(byte16, 4), [0b0001, 0b1011]);
    // }
}
