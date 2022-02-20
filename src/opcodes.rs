#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Op {
    Get(i8),
    Set(i8),
    LGet(u8),
    LSet(u8),
    Push(i32),
    Pop,
    Dup,
    Swap,

    JumpIfZero(u32),
    JumpIfNotZero(u32),
    Jump(u32),
    Ret,

    Lt,
    Gt,
    Eq,
    Le,
    Ge,
    Add,
    Sub,
    Div,
    Mul,
}
