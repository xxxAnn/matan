use crate::{things::{exp::Expression, funcs::RealRenderable}, obj::{CommandGrid, RenderCommand, WriteCommand}, alias::{DefaultRect, DefaultColor, DefaultPoint}};


pub struct RealNumberPairPlane {
    exps: Vec<Expression>,
    r: RealRenderable
}

type Clr = DefaultColor;
type Rct = DefaultRect;
type Pnt = DefaultPoint;

impl CommandGrid<Clr, Clr, Rct, Pnt, Vec<WriteCommand<Clr, Rct>>, Vec<RenderCommand<Clr, Pnt>>, Expression, RealRenderable> for RealNumberPairPlane {
    fn text_write_commands(&self, inst: u128) -> Vec<WriteCommand<Clr, Rct>> {
        todo!()
    }

    fn object_render_commands(&self, inst: u128) -> Vec<RenderCommand<Clr, Pnt>> {
        self.r.v
            .iter()
            .filter(|((a, b), _)| a<&inst && b<&inst)
                .map(|(_, z)| 
                    z.clone()
                )
            .collect()
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