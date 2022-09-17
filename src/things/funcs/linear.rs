use crate::obj::{RenderCommand, Object};

pub struct LinearFunction {
    v: Vec<((u128, u128), RenderCommand<(u8, u8, u8, u8), (i32, i32)>)>
}

pub enum LinearFunctionDescriptor {
    Angle(f32),
    Standard(f32, f32)
}

impl Object for LinearFunction {
    type Clr = (u8, u8, u8, u8);
    type Pnt = (i32, i32);
    type Params = LinearFunctionDescriptor;

    fn render(&self, inst: std::time::Instant) -> RenderCommand<Self::Clr, Self::Pnt> {
        todo!()
    }

    fn add_snapshot(&mut self, start: u64, length: std::time::Duration, params: Option<Self::Params>) {
        let m = (start as u128, (start as u128)+length.as_millis());
    } 
}