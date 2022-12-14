use sdl2::{pixels::Color, rect::Point};

use crate::obj::{RenderCommand, Object};

use crate::alias::{DefaultColor, DefaultPoint};

pub struct Function {
    pub v: Vec<((u128, u128), RenderCommand<DefaultColor, DefaultPoint>)>,
    screen: (u32, u32)
}

#[derive(Clone)]
pub enum FunctionDescriptor {
    Angle(AngularDescriptor),
    Standard(SlopeDescriptor)
}

#[derive(Clone)]
pub struct AngularDescriptor {
    angle: f32, // radians
    intercept: f32,
    width: f32
}

impl AngularDescriptor {
    pub fn new(angle: f32, intercept: f32, width: f32) -> Self {
        Self {
            angle,
            intercept,
            width
        }
    }
}

#[derive(Clone)]
pub struct SlopeDescriptor {
    slope: f32,
    intercept: f32,
    width: f32
}

impl SlopeDescriptor {
    pub fn new(slope: f32, intercept: f32, width: f32) -> Self {
        Self {
            slope,
            intercept,
            width
        }
    }
}

impl FunctionDescriptor {
    #[must_use]
    pub fn slope(&self) -> f32 {
        match self {
            Self::Angle(a) => {
                a.angle.tan()
            },
            Self::Standard(s) => {
                s.slope
            }
        }
    }
    #[must_use]
    pub const fn intercept(&self) -> f32 {
        match self {
            Self::Angle(a) => a.intercept,
            Self::Standard(s) => s.intercept
        }
    }
    #[must_use]
    pub const fn width(&self) -> f32 {
        match self {
            Self::Angle(a) => a.width,
            Self::Standard(s) => s.width
        }
    }
}

#[must_use]
pub fn combine_renderers<T, K>(o: Vec<&RenderCommand<T, K>>) -> RenderCommand<T, K>
where T: Into<Color> + Clone, K: Into<Point> + PartialEq + Clone {
    let mut v = Vec::new();
    //let bfr = std::time::Instant::now();
    for x in o {
        v.extend(x.points().to_vec().into_iter());
    }
    //println!("Combining renderers took {:?}ms", std::time::Instant::now().duration_since(bfr).as_millis());

    RenderCommand::new(v)
}

impl Object for Function {
    type Clr = DefaultColor;
    type Pnt = DefaultPoint;
    type Params = FunctionDescriptor;

    fn render(&self, inst: u128) -> RenderCommand<Self::Clr, Self::Pnt> {
        //let bfr = std::time::Instant::now();
        let r = combine_renderers(self.v
            .iter()
            .filter(|((a, b), _)| a<=&inst && &inst<b)
            .map(|(_, b)| {
                b
            })
            .collect()
        );
        //println!("Rendering Function took {:?}ms", std::time::Instant::now().duration_since(bfr).as_millis());
        r
    }

    fn add_snapshot(&mut self, start: u64, length: std::time::Duration, params: Option<Self::Params>) {
        let m = (u128::from(start), (u128::from(start))+length.as_millis());
        match params {
            Some(p) =>  
            self.v.push((
                m, 
                RenderCommand::from_linear_function(
                    p.slope(),
                    p.intercept(), 
                    p.width(), 
                    self.get_screen_data()
                )
            )),
            None => {},
        }
        
    } 
}   

impl Function {
    pub const fn new(screen: (u32, u32)) -> Self {
        Self {
            v: Vec::new(),
            screen
        }
    }

    const fn get_screen_data(&self) -> (u32, u32) {
        self.screen
    }
}