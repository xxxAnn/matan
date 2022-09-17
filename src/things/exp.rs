pub struct Expression {
    txt: String,
    pos: (i32, i32), 
    size: (u32, u32),
    font: &'static str,
    psize: u16,
    clr: (u8, u8, u8, u8)   
}

