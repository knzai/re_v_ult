pub mod viewer {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    use sdl2::gfx::primitives::DrawRenderer;
    use sdl2::pixels::Color;
    use sdl2::render::WindowCanvas;

    const COLORS: [Color; 8] = [
        Color::RGB(0, 0, 170),
        Color::RGB(85, 255, 85),
        Color::RGB(0, 170, 0),
        Color::RGB(85, 85, 85),
        Color::RGB(255, 255, 255),
        Color::RGB(170, 85, 0),
        Color::RGB(255, 85, 255),
        Color::RGB(0, 0, 0),
    ];

    pub fn process_map_bin(
        path: &str,
        canvas: &mut WindowCanvas,
    ) -> Result<(), Box<dyn std::error::Error>> {
        canvas.set_draw_color(Color::RGB(0, 0, 170));
        canvas.clear();

        let file = File::open(path)?;

        let mut reader = BufReader::with_capacity(84, file);

        let mut y = 0;

        loop {
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
                let nib1: u8 = byte >> 4;
                let nib2: u8 = byte & 0x0F;

                print!("{}", nib1);
                print!("{}", nib2);
                if nib1 != 0 {
                    canvas.pixel(x, y, COLORS[nib1 as usize])?;
                }
                if nib2 != 0 {
                    canvas.pixel(x + 1, y, COLORS[nib1 as usize])?;
                }
                x += 2;
            }
            println!();

            y += 1;
            reader.consume(buffer_length);
        }

        Ok(())
    }
}
