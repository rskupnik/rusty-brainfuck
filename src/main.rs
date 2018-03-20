enum Command {
    ShiftRight,
    ShiftLeft,
    Increment,
    Decrement,
    Output,
    Input,
    LoopStart,
    LoopEnd
}

fn interpret(cmd: &str) -> &str {
    match cmd {
        _ => "hello, world!"
    }
}

fn main() {
    let val = interpret("hi");
    println!("{}", val);
}
