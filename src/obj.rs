use crate::text;

use std::time;

use sdl2::{pixels::Color, rect::{Point, Rect}, ttf::Sdl2TtfContext, render::Canvas, video::Window, Sdl, VideoSubsystem, event::Event};

pub const RR: u128 = 100;

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

    fn process(&mut self, cvs: &mut Canvas<Window>, sdl: &Sdl, ttf: &Sdl2TtfContext, vis: &VideoSubsystem, ins: time::Instant) -> Result<(), String> {
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

    fn start(&mut self, title: &str, width: u32, height: u32) -> Result<(), String> {
        let sdl_context = sdl2::init()?;
        let video_subsys = sdl_context.video()?;
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    
        let window = video_subsys
            .window(
                title,
                width,
                height,
            )
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;
    
        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    
        self.setup(&mut canvas, &sdl_context, &ttf_context, &video_subsys);

        canvas.present();

        let ins = time::Instant::now();
    
        let mut events = sdl_context.event_pump()?;
    
        let mut refresh = time::Instant::now();

        'main: loop {

            for event in events.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'main,
                    _ => {}
                }
            }
            // only process every f millis
            if refresh.elapsed().as_millis() > RR {
                refresh = time::Instant::now();
                self.process(&mut canvas, &sdl_context, &ttf_context, &video_subsys, ins)?;
            }
        }
    
        Ok(())
    }
}

