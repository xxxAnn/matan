use crate::obj::{RenderCommand, Object};

use self::linear::{LinearFunction, combine_renderers};

pub mod linear;

pub struct RealRenderable {
    pub v: Vec<((u128, u128), RenderCommand<(u8, u8, u8, u8), (i32, i32)>)>
}

impl Object for RealRenderable {
    type Clr = (u8, u8, u8, u8);
    type Pnt = (i32, i32);
    type Params = RenderCommand<(u8, u8, u8, u8), (i32, i32)>;

    fn render(&self, inst: u128) -> RenderCommand<Self::Clr, Self::Pnt> {
       combine_renderers(self.v.iter().filter(|((a, b), _)| a<&inst && &inst<b).map(|(_, z)| z).collect())
    }

    fn add_snapshot(&mut self, start: u64, length: std::time::Duration, params: Option<Self::Params>) {
        let m = (start as u128, (start as u128)+length.as_millis());
        match params {
            Some(n) => self.v.push((m, n)),
            _ => {}
        }
    }
}

impl RealRenderable {
    pub fn new(o: &[LinearFunction], lookup_intervals: Vec<u128>, delay: u128) -> Self {
        let mut r = RealRenderable {
            v: vec!()
        };
        for i in lookup_intervals {
            r.v = (o.iter().map(|x| ((i, i+delay), x.render(i)))).collect();
        }
        
        r
    }
}