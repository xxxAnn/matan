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

impl Text for Expression {
    type Clr = (u8, u8, u8, u8);
    type Rct = (i32, i32, u32, u32);
    type Params = ExpressionUnit;

    fn render(&self, inst: u128) -> Vec<WriteCommand<Self::Clr, Self::Rct>> {
        self.v.iter().filter(|((a, b), _)| a<&inst && &inst<b).map(|(_, z)| WriteCommand::new(
            z.clr,
            (z.pos.0, z.pos.1, z.size.0, z.size.0),
            z.font,
            z.txt.clone(),
            z.psize
            
        )).collect()
    }

    fn add_snapshot(&mut self, start: u64, length: std::time::Duration, params: Option<Self::Params>) {
        let m = (start as u128, (start as u128)+length.as_millis());
        match params {
            Some(p) => self.v.push((m, p)),
            _ => {}
        }
        
    }
}