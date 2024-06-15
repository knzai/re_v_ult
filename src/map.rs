pub mod viewer {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    use sdl2::gfx::primitives::DrawRenderer;
    use sdl2::pixels::Color;
    use sdl2::render::WindowCanvas;

    //EGA colors by map nibble index
    const COLORS: [Color; 8] = [
        Color::RGB(0, 0, 170),     //blue
        Color::RGB(85, 255, 85),   //bright green
        Color::RGB(0, 170, 0),     //green
        Color::RGB(85, 85, 85),    //dark gray
        Color::RGB(255, 255, 255), //white
        Color::RGB(170, 85, 0),    //brown
        Color::RGB(255, 85, 255),  //bright magenta
        Color::RGB(0, 0, 0),       //black
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
                let nibs = crate::formats::Byte { byte: *byte }.nibbles();

                print!("{}", nibs[0]);
                print!("{}", nibs[1]);
                if nibs[0] != 0 {
                    canvas.pixel(x, y, COLORS[nibs[0] as usize])?;
                }
                if nibs[1] != 0 {
                    canvas.pixel(x + 1, y, COLORS[nibs[1] as usize])?;
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
