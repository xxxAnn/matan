use crate::{text, alias::DefaultPoint};

use std::time;

use crate::alias::DefaultColor;

use sdl2::{pixels::Color, rect::{Point, Rect}, ttf::Sdl2TtfContext, render::Canvas, video::Window, Sdl, VideoSubsystem, event::Event};

pub const RR: u128 = 30;

pub mod fonts {
    const SANS: &str = "OpenSans-Regular.ttf";
}

#[derive(Clone)]
pub struct RenderCommand<T, K> 
where T: Into<Color>, K: Into<Point> {
    pts: Vec<(T, K)>
}

impl<T, K> RenderCommand<T, K>
where T: Into<Color>, K: Into<Point> {
    pub fn points(&self) -> &[(T, K)] {
        &self.pts
    }
}

impl<T, K> RenderCommand<T, K>
where T: Into<Color>, K: Into<Point> {
    pub fn new(pts: Vec<(T, K)>) -> Self {
        Self {
            pts
        }
    }
}

#[derive(Clone)]
pub struct WriteCommand<T, K>
where T: Into<Color>, K: Into<Rect> {
    clr: T,
    rct: K,
    pth: &'static str,
    txt: String,
    psize: u16
}

impl<T, K> WriteCommand<T, K>
where T: Into<Color>, K: Into<Rect> {
    pub const fn new(clr: T, rct: K, pth: &'static str, txt: String, psize: u16) -> Self {
        Self {
            clr,
            rct,
            pth,
            txt,
            psize
        }
    }
}

/// Trait for every object that can be rendered
pub trait Object {
    type Clr: Into<Color>;
    type Pnt: Into<Point>;
    type Params;
    /// Returns a command to Render the object at the specified instant
    fn render(&self, inst: u128) -> RenderCommand<Self::Clr, Self::Pnt>;
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
    fn render(&self, inst: u128) -> Vec<WriteCommand<Self::Clr, Self::Rct>>;
    /// Params is a struct containing parameters to generate text
    /// and rect size.
    /// If length is zero this snapshot will always be rendered.
    /// If multiple snapshots overlap, they will be rendered
    /// from earliest to latest.
    fn add_snapshot(&mut self, start: u64, length: time::Duration, params: Option<Self::Params>);
}

pub trait CommandGrid<T, J, K, V, R, U, X, Z>
where T: Into<Color>, J: Into<Color>, K: Into<Rect>, V: Into<Point>,
X: Text, Z: Object {
    /// Returns an Iterator over `WriteCommand` objects
    /// at the specified instant.
    fn text_write_commands(&self, inst: u128) -> Vec<WriteCommand<T, K>>;
    /// Returns an Iterator over `RenderCommand` objects
    /// at the specified instant.
    fn object_render_commands(&self, inst: u128) -> Vec<RenderCommand<J, V>>; 
    /// Adds a Text object to the grid.
    fn add_text(&mut self, t: X);
    /// Adds a Renderable object to the grid.
    fn add_object(&mut self, o: Z);
    /// Setup the Canvas  before drawing anything on it.
    fn setup(&mut self, cvs: &mut Canvas<Window>, sdl: &Sdl, ttf: &Sdl2TtfContext, vis: &VideoSubsystem);

    fn process(&mut self, cvs: &mut Canvas<Window>, sdl: &Sdl, ttf: &Sdl2TtfContext, vis: &VideoSubsystem, ins: u128) -> Result<(), String> {
        self.setup(cvs, sdl, ttf, vis);
        for r in self.object_render_commands(ins) {
            for (a, b) in r.pts {
                cvs.set_draw_color(a);
                cvs.draw_point(b)?;
            }
        }
        cvs.present();
        for t in self.text_write_commands(ins) {
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
                if let Event::Quit { .. } = event { break 'main }
            }
            // only process every RR millis
            if refresh.elapsed().as_millis() >= RR {
                refresh = time::Instant::now();
                self.process(&mut canvas, &sdl_context, &ttf_context, &video_subsys, 
                    time::Instant::now().duration_since(ins).as_millis()
                )?;
            }
        }
    
        Ok(())
    }
}

impl RenderCommand<DefaultColor, DefaultPoint> {
    pub fn from_linear_function(m: f32, b: f32, width: f32, screen: (u32, u32)) -> Self {
        let mut v = Vec::new();
        let w = screen.0 as i32;
        let h = screen.1 as i32;
        let ksq = width.powi(2);
        let disc = (ksq*(1.+m.powi(2))).sqrt();
        let n = (0u8, 0u8, 0u8, 1u8);

        //let bfr = time::Instant::now();

        for x in 0i32..w {
            let cx = x-w/2;
            let ray = m.mul_add(cx as f32, b);
            v.append(&mut ((ray-disc).floor() as i32..(ray+disc).floor() as i32).into_iter().map(|y| {
                (n, (x, -y+h/2))
            }).collect());
        }

        //println!("Solving for y = {}x + {} took {:?}??s", &m, &b, time::Instant::now().duration_since(bfr).as_micros());

        Self {
            pts: v
        }
    }
}