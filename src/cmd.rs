//! Contains a single enum, `Command` which describes all the possible
//! commands available.

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

