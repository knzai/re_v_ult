use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod formats;
mod map;
mod tiles;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        //.window("map viewer", 168, 168)
        .window("map viewer", 850, 850)
        //.allow_highdpi()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");
    canvas.clear();

    //

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
                Event::KeyDown {
                    keycode: Some(Keycode::M),
                    ..
                } => {
                    let _ = map::viewer::process_map_bin("./assets/game/MAP.BIN", &mut canvas);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::T),
                    ..
                } => {
                    let _ = tiles::process_cga_tile_bin("./assets/game/CGATILES.BIN", &mut canvas);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::E),
                    ..
                } => {
                    let _ = tiles::process_ega_tile_bin("./assets/game/EGATILES.BIN", &mut canvas);
                }
                _ => {}
            }
        }
    }
    Ok(())
}
