use bitvec::prelude::*;

pub mod cga {
    use bitvec::prelude::*;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    use sdl2::gfx::primitives::DrawRenderer;
    use sdl2::pixels::Color;
    use sdl2::render::WindowCanvas;

    const PALETTE1: [Color; 4] = [
        Color::BLACK,                 //black
        Color::RGB(0x00, 0xAA, 0xAA), //cyan
        Color::RGB(0xAA, 0x00, 0xAA), //magenta
        Color::RGB(0xAA, 0xAA, 0xAA), //gray
    ];
    const PALETTE1I: [Color; 4] = [
        Color::BLACK,                 //black
        Color::RGB(0x55, 0xFF, 0xFF), //bright cyan
        Color::RGB(0xFF, 0x55, 0xFF), //bright magenta
        Color::WHITE,                 //white
    ];

    pub fn out_cgatiles(
        path: &str,
        canvas: &mut WindowCanvas,
    ) -> Result<(), Box<dyn std::error::Error>> {
        canvas.clear();

        let file = File::open(path)?;

        let mut reader = BufReader::with_capacity(64, file);

        let mut y = 0_u16;
        loop {
            let buffer = reader.fill_buf()?;

            let buffer_length = buffer.len();

            if buffer_length == 0 {
                canvas.present();
                break;
            }
            out_16x16(&buffer, canvas, &mut y).expect("broke in tile processing");
            reader.consume(buffer_length);
        }

        Ok(())
    }

    pub fn out_16x16(
        buffer: &[u8],
        canvas: &mut WindowCanvas,
        y: &mut u16,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for (i, tastes) in buffer.view_bits::<Msb0>().chunks(2).enumerate() {
            let x = i % 16;
            if x == 0 {
                println!();
                *y += 1;
            }
            let color_index = tastes.load::<u8>();
            print!("{}", color_index);
            canvas.pixel(
                x.try_into().unwrap(),
                (*y).try_into().unwrap(),
                PALETTE1[color_index as usize],
            )?;
        }

        Ok(())
    }

    pub fn indices(buffer: &[u8]) -> Vec<u8> {
        buffer
            .view_bits::<Msb0>()
            .chunks(2)
            .map(|m| m.load::<u8>())
            .collect()
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
#[test]
fn test_to_indices() {
    let data: u128 = 0xFF_FF_FF_FF_FD_7F_F6_9F_F6_9F_FD_7F_FF_FF_FF_FF;
    let buffer = data.to_be_bytes();
    assert_eq!(
        cga::indices(&buffer),
        [
            3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 1, 1, 3, 3, 3, 3, 3, 1, 2, 2,
            1, 3, 3, 3, 3, 1, 2, 2, 1, 3, 3, 3, 3, 3, 1, 1, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
            3, 3, 3, 3, 3, 3
        ]
    );
}
