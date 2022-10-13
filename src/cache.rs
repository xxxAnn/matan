use sdl2::{pixels::Color, rect::Point};

use crate::obj::RenderCommand;

pub struct Cache<T, K> 
where T: Into<Color>, K: Into<Point> {
    t: std::collections::HashMap<(f32, f32, f32), RenderCommand<T, K>>
}

impl<T, K> Cache<T, K> 
where T: Into<Color>, K: Into<Point> {
    pub fn get(&mut self, a: (f32, f32, f32)) -> Result<RenderCommand<T, K>, String> {
        self.t.get(a)
    }
}