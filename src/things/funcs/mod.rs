use crate::{obj::{RenderCommand, Object}, alias::DefaultPoint};

use self::linear::combine_renderers;

use crate::alias::DefaultColor;

pub mod linear;

pub struct RealRenderable {
    pub v: Vec<((u128, u128), RenderCommand<DefaultColor, DefaultPoint>)>
}

impl Object for RealRenderable {
    type Clr = DefaultColor;
    type Pnt = DefaultPoint;
    type Params = RenderCommand<DefaultColor, DefaultPoint>;

    fn render(&self, inst: u128) -> RenderCommand<Self::Clr, Self::Pnt> {
       combine_renderers(self.v.iter().filter(|((a, b), _)| a<&inst && &inst<b).map(|(_, z)| z).collect())
    }

    fn add_snapshot(&mut self, start: u64, length: std::time::Duration, params: Option<Self::Params>) {
        let m = (u128::from(start) as u128, (u128::from(start))+length.as_millis());
        if let Some(n) = params { self.v.push((m, n)) }
    }
}

impl RealRenderable {
    #[must_use]
    pub fn new(o: &[linear::Function], lookup_intervals: Vec<u128>, delay: u128) -> Self {
        let mut r = Self {
            v: Vec::new()
        };
        for i in lookup_intervals {
            r.v = (o.iter().map(|x| ((i, i+delay), x.render(i)))).collect();
        }
        
        r
    }
}