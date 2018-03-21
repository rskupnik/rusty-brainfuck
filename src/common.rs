#[derive(Debug,PartialEq)]
pub enum Command {
    ShiftRight,
    ShiftLeft,
    Increment,
    Decrement,
    Output,
    Input(u8),
    LoopStart,
    LoopEnd,
    Unknown
}

impl PartialEq for Command::Input {
    fn eq(&self, other: &Command::Input) -> bool {
        match (self, other) {
            (&Command::Input(ref a), &Command::Input(ref b)) => a == b,
            _ => false
        }
    }
}
