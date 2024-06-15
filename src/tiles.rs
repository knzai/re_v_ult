use std::fs::File;
use std::io::{BufRead, BufReader};

use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

//CGA colors
const CGACOLORS: [Color; 4] = [
    Color::RGB(0, 0, 0),       //black
    Color::RGB(0, 170, 170),   //cyan
    Color::RGB(170, 0, 170),   //magenta
    Color::RGB(170, 170, 170), //gray
];

pub fn process_cga_tile_bin(
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
        let mut x = 0;
        for byte in buffer {
            let coups = crate::formats::Byte { byte: *byte }.couplets();
            if x % 4 == 0 {
                println!();
                y += 1;
                x = 0;
            }
            for i in 0..4 {
                let c = coups[i as usize];
                print!("{:02b}", c);
                //print!("{:?}", c);
                canvas.pixel(4 * x + i, y, CGACOLORS[c as usize])?;
            }
            x += 1;
        }
        reader.consume(buffer_length);
    }

    Ok(())
}
