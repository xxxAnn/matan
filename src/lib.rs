#![allow(dead_code, unused_imports)]
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{TextureCreator, Canvas};
use sdl2::video::{WindowContext, Window};
use sdl2::ttf::{self, Sdl2TtfContext};

mod obj;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

pub fn text<T, K>(
    ttfc: &Sdl2TtfContext, cvs: &mut Canvas<Window>, 
    pth: &str, clr: T, psize: u16, rct: K, txt: &str
) -> Result<(), String>
where T: Into<Color>, K: Into<Option<Rect>> {
    let texture_creator = cvs.texture_creator();
    let mut font = ttfc.load_font(pth, psize)?;
    font.set_style(sdl2::ttf::FontStyle::BOLD);
    let surface = font
        .render(txt)
        .blended(clr)
        .map_err(|e| e.to_string())?;
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    cvs.copy(&texture, None, rct)?;

    cvs.present();

    Ok(())
}

pub fn start(title: &str) -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

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

    for i in 0..(SCREEN_WIDTH/3)  {
        for k in 0..(SCREEN_HEIGHT/3) {
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
