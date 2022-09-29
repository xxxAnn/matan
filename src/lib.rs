#![allow(dead_code, unused_variables)]
extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::ttf::Sdl2TtfContext;

mod obj;
pub mod things;

pub mod alias {
    pub type DefaultColor = (u8, u8, u8, u8);
    pub type DefaultPoint = (i32, i32);
    pub type DefaultRect = (i32, i32, u32, u32);
}

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
