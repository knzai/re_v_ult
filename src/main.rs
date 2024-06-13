use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use std::fs::File;
use std::io::{BufRead, BufReader};

use sdl2::gfx::primitives::DrawRenderer;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const BUFFER_SIZE: usize = 84;

fn read_file_in_byte_chunks(
    path: &str,
    canvas: &mut WindowCanvas,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(path)?;

    let mut reader = BufReader::with_capacity(BUFFER_SIZE, file);

    let mut y = 0;

    loop {
        y += 1;
        let buffer = reader.fill_buf()?;

        let buffer_length = buffer.len();

        // BufRead could not read any bytes.
        // The file must have completely been read.
        if buffer_length == 0 {
            canvas.present();
            break;
        }
        let mut x = 0;
        for byte in buffer {
            let nib1 = byte >> 4;
            let nib2 = byte & 0x0F;

            //print!("{}",nib1);
            if nib1 != 0 {
                canvas.pixel(2 * x, y, Color::RGB(0, 255, 0))?;
            }
            if nib2 != 0 {
                canvas.pixel(2 * x + 1, y, Color::RGB(0, 255, 0))?;
            }

            //canvas.pixel(x, y, 0xFF000FFu32)?;
            //canvas.present();
            //canvas.pixel(x, i as i16, 0xFF000FFu32)?;
            //print!("{}",nib1);
            //print!("{}",nib2);
            x += 1;
        }
        //println!();

        // All bytes consumed from the buffer
        // should not be read again.
        y += 1;
        reader.consume(buffer_length);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("game tutorial", 336, 336)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");

    canvas.set_draw_color(Color::RGB(0, 0, 255));
    canvas.clear();

    read_file_in_byte_chunks("./assets/game/MAP.BIN", &mut canvas);

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }
    }
    Ok(())
}
