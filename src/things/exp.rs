use sdl2::{pixels::Color, rect::Rect};

use crate::obj::{Text, WriteCommand};

pub struct ExpressionUnit {
    txt: String,
    pos: (i32, i32), 
    size: (u32, u32),
    font: &'static str,
    psize: u16,
    clr: (u8, u8, u8, u8)   
}

pub struct Expression {
    v: Vec<((u128, u128), ExpressionUnit)>
}

pub fn combine_renderers<T, K>(o: Vec<&WriteCommand<T, K>>) -> WriteCommand<T, K>
where T: Into<Color> + Clone, K: Into<Rect> + PartialEq + Clone {
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

impl Text for Expression {
    type Clr = (u8, u8, u8, u8);
    type Rct = (i32, i32, u32, u32);
    type Params = ExpressionUnit;

    fn render(&self, inst: u128) -> crate::obj::WriteCommand<Self::Clr, Self::Rct> {
        combine_writers(self.v.into_iter().filter(|(a, _)| a<&inst && &inst<b).map(|(_, b)| WriteCommand::new(
            txt: b.txt,
            pos: b.pos,
            size: b.size,
            font: b.font,
            psize: b.psize,
            clr: b.clr
        )))
    }

    fn add_snapshot(&mut self, start: u64, length: std::time::Duration, params: Option<Self::Params>) {
        let m = (start as u128, (start as u128)+length.as_millis());
        match params {
            Some(p) => self.v.push((m, p)),
            _ => {}
        }
        
    }
}