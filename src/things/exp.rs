use crate::{obj::{Text, WriteCommand}, alias::{DefaultRect, DefaultPoint}};
use crate::alias::DefaultColor;

pub struct ExpressionUnit {
    txt: String,
    pos: DefaultPoint, 
    size: (u32, u32),
    font: &'static str,
    psize: u16,
    clr: DefaultColor   
}

pub struct Expression {
    v: Vec<((u128, u128), ExpressionUnit)>
}

impl Text for Expression {
    type Clr = DefaultColor;
    type Rct = DefaultRect;
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
        let m = (u128::from(start), (u128::from(start))+length.as_millis());
        if let Some(p) = params { self.v.push((m, p)) }
    }
}