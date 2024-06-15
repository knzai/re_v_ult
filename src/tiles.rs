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

//EGA colors
const EGACOLORS: [Color; 16] = [
    Color::RGB(0, 0, 170),     //blue
    Color::RGB(85, 255, 85),   //bright green
    Color::RGB(0, 170, 0),     //green
    Color::RGB(85, 85, 85),    //dark gray
    Color::RGB(255, 255, 255), //white
    Color::RGB(170, 85, 0),    //brown
    Color::RGB(255, 85, 255),  //bright magenta
    Color::RGB(0, 0, 0),       //black
    Color::RGB(0, 0, 170),     //blue
    Color::RGB(85, 255, 85),   //bright green
    Color::RGB(0, 170, 0),     //green
    Color::RGB(85, 85, 85),    //dark gray
    Color::RGB(255, 255, 255), //white
    Color::RGB(170, 85, 0),    //brown
    Color::RGB(255, 85, 255),  //bright magenta
    Color::RGB(0, 0, 0),       //black
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

pub fn process_ega_tile_bin(
    path: &str,
    canvas: &mut WindowCanvas,
) -> Result<(), Box<dyn std::error::Error>> {
    canvas.clear();

    let file = File::open(path)?;

    let mut reader = BufReader::with_capacity(4, file);

    let mut y = 0;

    loop {
        let buffer = reader.fill_buf()?;

        let buffer_length = buffer.len();

        if buffer_length == 0 {
            canvas.present();
            break;
        }
        let mut pixels = [0; 8];
        for (i, byte) in buffer.iter().enumerate() {
            (0..8)
                .rev()
                .for_each(|n| pixels[n] += 2 ^ i as u8 * ((byte >> n) & 1));
        }
        println!("{:?}", pixels);
        for (i, p) in pixels.iter().enumerate() {
            canvas.pixel(i as i16, y, EGACOLORS[*p as usize])?;
        }
        y += 1;
        reader.consume(buffer_length);
    }
    canvas.present();

    Ok(())
}
