use std::slice::{Iter, IterMut};

use crate::{things::{exp::Expression, funcs::RealRenderable}, obj::{CommandGrid, RenderCommand, WriteCommand}};


// for all practical purposes 'a = 'static
pub struct RealNumberPairPlane {
    exps: Vec<Expression>,
    r: RealRenderable
}

type Clr = (u8, u8, u8, u8);
type Rct = (i32, i32, u32, u32);
type Pnt = (i32, i32);

impl CommandGrid<Clr, Clr, Rct, Pnt, Vec<WriteCommand<Clr, Rct>>, Vec<RenderCommand<Clr, Pnt>>, Expression, RealRenderable> for RealNumberPairPlane {
    fn text_write_commands(&self, inst: std::time::Instant) -> Vec<WriteCommand<Clr, Rct>> {
        todo!()
    }

    fn object_render_commands(&self, inst: std::time::Instant) -> Vec<RenderCommand<Clr, Pnt>> {
        todo!()
    }

    fn add_text(&mut self, t: Expression) {
        todo!()
    }

    fn add_object(&mut self, o: RealRenderable) {
        todo!()
    }

    fn setup(&mut self, cvs: &mut sdl2::render::Canvas<sdl2::video::Window>, sdl: &sdl2::Sdl, ttf: &sdl2::ttf::Sdl2TtfContext, vis: &sdl2::VideoSubsystem) {
        todo!()
    }
}