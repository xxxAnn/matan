use crate::obj::Text;

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

    fn render(&self, inst: std::time::Instant) -> crate::obj::WriteCommand<Self::Clr, Self::Rct> {
        todo!()
    }

    fn add_snapshot(&mut self, start: u64, length: std::time::Duration, params: Option<Self::Params>) {
        let m = (start as u128, (start as u128)+length.as_millis());
        match params {
            Some(p) => self.v.push((m, p)),
            _ => {}
        }
        
    }
}