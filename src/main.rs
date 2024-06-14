use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod map;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        //.window("map viewer", 168, 168)
        .window("map viewer", 168, 168)
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");
    canvas.clear();

    let _ = map::viewer::process_map_bin("./assets/game/MAP.BIN", &mut canvas);
    //process_cga_tile_bin("./assets/game/CGATILES.BIN", &mut canvas);

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
