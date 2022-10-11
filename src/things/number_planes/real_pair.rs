use sdl2::pixels::Color;

use crate::{things::{exp::Expression, funcs::RealRenderable}, obj::{CommandGrid, RenderCommand, WriteCommand, Text}, alias::{DefaultRect, DefaultColor, DefaultPoint}};


pub struct RealNumberPairPlane {
    exps: Vec<Expression>,
    r: Option<RealRenderable>
}

type Clr = DefaultColor;
type Rct = DefaultRect;
type Pnt = DefaultPoint;

impl RealNumberPairPlane {
    pub fn new() -> Self{
        Self {
            exps: vec![],
            r: None
        }
    }
}

impl CommandGrid<Clr, Clr, Rct, Pnt, Vec<WriteCommand<Clr, Rct>>, Vec<RenderCommand<Clr, Pnt>>, Expression, RealRenderable> for RealNumberPairPlane {
    fn text_write_commands(&self, inst: u128) -> Vec<WriteCommand<Clr, Rct>> {
        self.exps.iter().map(|x| x.render(inst)).collect::<Vec<Vec<WriteCommand<Clr, Rct>>>>().concat()
    }

    fn object_render_commands(&self, inst: u128) -> Vec<RenderCommand<Clr, Pnt>> {
        if let Some(k) = &self.r {
            k.v
            .iter()
            .filter(|((a, b), _)| {
                a<&inst && &inst<b
            } )
                .map(|(_, z)| 
                    z.clone()
                )
            .collect()
        } else {
            vec![]
        }
    }

    fn add_text(&mut self, t: Expression) {
        self.exps.push(t);
    }

    fn add_object(&mut self, o: RealRenderable) {
        self.r = Some(o);
    }

    fn setup(&mut self, cvs: &mut sdl2::render::Canvas<sdl2::video::Window>, sdl: &sdl2::Sdl, ttf: &sdl2::ttf::Sdl2TtfContext, vis: &sdl2::VideoSubsystem) {
        cvs.set_draw_color(Color::RGB(255, 255, 255));
        cvs.clear();
    }
}

