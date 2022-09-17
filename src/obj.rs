use crate::text;

use std::time;

use sdl2::{pixels::Color, rect::{Point, Rect}, ttf::Sdl2TtfContext, render::Canvas, video::Window, Sdl, VideoSubsystem};

pub mod fonts {
    const SANS: &'static str = "OpenSans-Regular.ttf";
}

pub struct RenderCommand<T, K> 
where T: Into<Color>, K: Into<Point> {
    pts: Vec<(T, K)>
}

pub struct WriteCommand<T, K>
where T: Into<Color>, K: Into<Rect> {
    clr: T,
    rct: K,
    pth: &'static str,
    txt: String,
    psize: u16
}

/// Trait for every object that can be rendered
pub trait Object {
    type Clr: Into<Color>;
    type Pnt: Into<Point>;
    type Params;
    /// Returns a command to Render the object at the specified instant
    fn render(&self, inst: time::Instant) -> RenderCommand<Self::Clr, Self::Pnt>;
    /// Params is a struct containing parameters to generate points and colors.
    /// If length is zero this snapshot will always be rendered.
    /// If multiple snapshots overlap, they will be rendered
    /// from earliest to latest.
    fn add_snapshot(&mut self, start: u64, length: time::Duration, params: Option<Self::Params>);
}

// Trait for every text object that can be written
pub trait Text {
    type Clr: Into<Color>;
    type Rct: Into<Rect>;
    type Params;
    /// Returns a command to Write the text at the specified instant
    fn render(&self, inst: time::Instant) -> WriteCommand<Self::Clr, Self::Rct>;
    /// Params is a struct containing parameters to generate text
    /// and rect size.
    /// If length is zero this snapshot will always be rendered.
    /// If multiple snapshots overlap, they will be rendered
    /// from earliest to latest.
    fn add_snapshot(&mut self, start: u64, length: time::Duration, params: Option<Self::Params>);
}

pub trait CommandGrid<T, J, K, V, R, U, X, Z>
where T: Into<Color>, J: Into<Color>, K: Into<Rect>, V: Into<Point>, 
R: Iterator<Item = WriteCommand<T, K>>, 
U: Iterator<Item = RenderCommand<J, V>>,
X: Text, Z: Object {
    /// Returns an Iterator over WriteCommand objects
    /// at the specified instant.
    fn text_write_commands(&self, inst: time::Instant) -> R;
    /// Returns an Iterator over RenderCommand objects
    /// at the specified instant.
    fn object_render_commands(&self, inst: time::Instant) -> U; 
    /// Adds a Text object to the grid.
    fn add_text(&mut self, t: X);
    /// Adds a Renderable object to the grid.
    fn add_object(&mut self, o: Z);
    /// Setup the Canvas  before drawing anything on it.
    fn setup(&mut self, cvs: &mut Canvas<Window>, sdl: &Sdl, ttf: &Sdl2TtfContext, vis: &VideoSubsystem);

    fn process(&mut self, cvs: &mut Canvas<Window>, sdl: &Sdl, ttf: &Sdl2TtfContext, vis: &VideoSubsystem) -> Result<(), String> {
        let ins = time::Instant::now();

        self.setup(cvs, sdl, ttf, vis);

        for r in self.object_render_commands(ins.clone()) {
            for (a, b) in r.pts {
                cvs.set_draw_color(a);
                cvs.draw_point(b)?;
            }
        }
        cvs.present();
        for t in self.text_write_commands(ins.clone()) {
            text(ttf, cvs, t.pth, t.clr, t.psize, Some(t.rct.into()), &t.txt)?;
        }
        Ok(())
    }
}

