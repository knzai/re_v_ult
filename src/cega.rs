use bitvec::prelude::*;

pub mod cga {
    use bitvec::prelude::*;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    use sdl2::gfx::primitives::DrawRenderer;
    use sdl2::pixels::Color;
    use sdl2::render::WindowCanvas;

    const COLORS: [Color; 4] = [
        Color::RGB(0, 0, 0),       //black
        Color::RGB(0, 170, 170),   //cyan
        Color::RGB(170, 0, 170),   //magenta
        Color::RGB(170, 170, 170), //gray
    ];

    pub fn out16x16(
        path: &str,
        canvas: &mut WindowCanvas,
    ) -> Result<(), Box<dyn std::error::Error>> {
        canvas.clear();

        let file = File::open(path)?;

        let mut reader = BufReader::with_capacity(64, file);

        let mut y = 0;
        loop {
            let buffer = reader.fill_buf()?;

            let buffer_length = buffer.len();

            if buffer_length == 0 {
                canvas.present();
                break;
            }

            for (i, tastes) in buffer.view_bits::<Msb0>().chunks(2).enumerate() {
                let x = i % 16;
                if x == 0 {
                    println!();
                    y += 1;
                }
                let color_index = tastes.load::<u8>();
                print!("{}", color_index);
                canvas.pixel(x.try_into().unwrap(), y, COLORS[color_index as usize])?;
            }
            reader.consume(buffer_length);
        }

        Ok(())
    }
}

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
