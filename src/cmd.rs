#[derive(Debug,PartialEq)]
pub enum Command {
    ShiftRight,
    ShiftLeft,
    Increment,
    Decrement,
    Output,
    Input,
    LoopStart,
    LoopEnd,
    Unknown
}

