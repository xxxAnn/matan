use sdl2::{pixels::Color, rect::Point};

use crate::obj::{RenderCommand, Object};

pub struct LinearFunction {
    pub v: Vec<((u128, u128), RenderCommand<(u8, u8, u8, u8), (i32, i32)>)>,
    screen: (u32, u32)
}

pub enum LinearFunctionDescriptor {
    Angle(AngularLinearFunctionDescriptor),
    Standard(InterceptLinearFunctionDescriptor)
}

pub struct AngularLinearFunctionDescriptor {
    angle: f32,
    intercept: f32,
    width: f32
}

pub struct InterceptLinearFunctionDescriptor {
    slope: f32,
    intercept: f32,
    width: f32
}

impl LinearFunctionDescriptor {
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

    pub fn intercept(&self) -> f32 {
        match self {
            Self::Angle(a) => a.intercept,
            Self::Standard(s) => s.intercept
        }
    }

    pub fn width(&self) -> f32 {
        match self {
            Self::Angle(a) => a.width,
            Self::Standard(s) => s.width
        }
    }
}

pub fn combine_renderers<T, K>(o: Vec<&RenderCommand<T, K>>) -> RenderCommand<T, K>
where T: Into<Color> + Clone, K: Into<Point> + PartialEq + Clone {
    let mut alr = vec!();
    let mut v = vec!();
    for x in o {
        for (a, b) in x.points() {
            if !alr.contains(&b) {
                alr.push(b);
                v.push((a.clone(), b.clone()))
            }
        }
    }

    RenderCommand::new(v)
}

impl Object for LinearFunction {
    type Clr = (u8, u8, u8, u8);
    type Pnt = (i32, i32);
    type Params = LinearFunctionDescriptor;

    fn render(&self, inst: u128) -> RenderCommand<Self::Clr, Self::Pnt> {
        combine_renderers(self.v
            .iter()
            .filter(|((a, b), _)| a<&inst && &inst<b)
            .map(|(_, b)| b)
            .collect()
        )
    }

    fn add_snapshot(&mut self, start: u64, length: std::time::Duration, params: Option<Self::Params>) {
        let m = (start as u128, (start as u128)+length.as_millis());
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

impl LinearFunction {
    fn new(screen: (u32, u32)) -> Self {
        return Self {
            v: vec!(),
            screen
        }
    }

    fn get_screen_data(&self) -> (u32, u32) {
        self.screen
    }
}