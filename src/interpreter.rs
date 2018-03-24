use common::Command;
use std::collections::HashMap;

pub fn interpret(cmd: &str) -> Command {
    interpret_char(cmd.chars().next().unwrap())
}

fn interpret_char(cmd: char) -> Command {
    match cmd {
        '>' => Command::ShiftRight,
        '<' => Command::ShiftLeft,
        '+' => Command::Increment,
        '-' => Command::Decrement,
        '.' => Command::Output,
        ',' => Command::Input,
        '[' => Command::LoopStart,
        ']' => Command::LoopEnd,
        _ => Command::Unknown
    }
}

pub fn translate(program: &str) -> HashMap<usize, Command> {
    let mut output = HashMap::new();
    let mut counter: usize = 0;
    for c in program.chars() {
        output.insert(counter, interpret_char(c));
        counter += 1;
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::Command;
    use std::collections::HashMap;

    #[test]
    fn translate_into_map() {
        let program = ">.<";
        let result: HashMap<usize, Command> = translate(program);
        assert_eq!(3, result.len());
        assert_eq!(&Command::ShiftRight, result.get(&0).expect("should be ShiftRight"));
        assert_eq!(&Command::Output, result.get(&1).expect("should be Output"));
        assert_eq!(&Command::ShiftLeft, result.get(&2).expect("should be ShiftLeft"));
    }

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
