use crate::obj::{RenderCommand, Object};

use self::linear::LinearFunction;

pub mod linear;

pub struct RealRenderable {
    v: Vec<((u128, u128), RenderCommand<(u8, u8, u8, u8), (i32, i32)>)>
}

impl Object for RealRenderable {
    type Clr = (u8, u8, u8, u8);
    type Pnt = (i32, i32);
    type Params = RenderCommand<(u8, u8, u8, u8), (i32, i32)>;

    fn render(&self, inst: std::time::Instant) -> RenderCommand<Self::Clr, Self::Pnt> {
        todo!()
    }

    fn add_snapshot(&mut self, start: u64, length: std::time::Duration, params: Option<Self::Params>) {
        
    }
}

impl Into<RealRenderable> for LinearFunction {
    fn into(self) -> RealRenderable {
        RealRenderable { v: self.v }
    }
}