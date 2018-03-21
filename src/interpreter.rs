use common::Command;

pub fn interpret(cmd: &str) -> Command {
    match cmd {
        ">" => Command::ShiftRight,
        "<" => Command::ShiftLeft,
        "+" => Command::Increment,
        "-" => Command::Decrement,
        "." => Command::Output,
        "," => Command::Input,
        "[" => Command::LoopStart,
        "]" => Command::LoopEnd,
        _ => Command::Unknown
    }
}

#[cfg(test)]
mod tests {
    use common::Command;
    use interpreter::interpret;

    #[test]
    fn interprets_shift_right_command() {
        let cmd = interpret(">");
        assert_eq!(cmd, Command::ShiftRight);
    }

    #[test]
    fn interprets_shift_left_command() {
        let cmd = interpret("<");
        assert_eq!(cmd, Command::ShiftLeft);
    }

    #[test]
    fn interprets_increment_command() {
        let cmd = interpret("+");
        assert_eq!(cmd, Command::Increment);
    }

    #[test]
    fn interprets_decrement_command() {
        let cmd = interpret("-");
        assert_eq!(cmd, Command::Decrement);
    }

    #[test]
    fn interprets_output_command() {
        let cmd = interpret(".");
        assert_eq!(cmd, Command::Output);
    }

    #[test]
    fn interprets_input_command() {
        let cmd = interpret(",");
        assert_eq!(cmd, Command::Input);
    }

    #[test]
    fn interprets_loop_start_command() {
        let cmd = interpret("[");
        assert_eq!(cmd, Command::LoopStart);
    }

    #[test]
    fn interprets_loop_end_command() {
        let cmd = interpret("]");
        assert_eq!(cmd, Command::LoopEnd);
    }

}
