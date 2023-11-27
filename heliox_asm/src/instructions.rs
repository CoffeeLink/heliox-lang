#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub enum Instruction {
    NoOperation,

    //Memory
    Move,

    //Math
    Add,
    Sub,
    And,
    Or,
    Xor,
    Not,
    ShiftLeft,
    ShiftRight,

    // Flow Control
    Jump,
    JumpEquals,
    JumpNotEquals,
    JumpGreaterThan,
    JumpLessThan,
    JumpGreaterThanEqual,
    JumpLessThanEqual,

    Call,
    CallEquals,
    CallNotEquals,
    CallGreaterThan,
    CallLessThan,
    CallGreaterThanEqual,
    CallLessThanEqual,

    Return,
    ReturnEquals,
    ReturnNotEquals,
    ReturnGreaterThan,
    ReturnLessThan,
    ReturnGreaterThanEqual,
    ReturnLessThanEqual,

    // Stack
    Push,
    Pop,
    Peek,

    //System Calls
    Halt,
    Reset,
    ResetStack,
    ResetMemory,
}