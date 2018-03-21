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

/*impl PartialEq for Command {
    fn eq(&self, other: &Command) -> bool {
        match (self, other) {
            (&Command::Input(ref a), &Command::Input(ref b)) => a == b,
            (&Command::Input(ref a), &Command) => false,
            (&Command, &Command::Input(ref a)) => false,
            _ => false
        }
    }
}*/
