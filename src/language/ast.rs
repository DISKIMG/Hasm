#[derive(Debug, Clone)]
pub enum Node {
    Mov { dst: String, src: String },
    Jmp { label: String },
    Int { num: u8 },
    Ret,
    Label(String),
    RawAsm(String),
}
