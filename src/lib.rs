extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

pub fn start(title: &str) -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;
    let window = video_subsys
        .window(
            title,
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
        )
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color((0, 0, 0));
    canvas.clear();
    canvas.set_draw_color((255, 0, 0));
    for i in 1..(SCREEN_WIDTH/3)  {
        for k in 1..(SCREEN_HEIGHT/3) {
            canvas.draw_point((i as i32, k as i32)).unwrap();
        }
    }
    canvas.present();

    let mut events = sdl_context.event_pump()?;

    'main: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                _ => {}
            }
        }
    }

    Ok(())
}
