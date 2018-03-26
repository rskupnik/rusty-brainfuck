use cmd::Command;
use std::vec::Vec;

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

pub fn translate(program: &str) -> Vec<(usize, Command)> {
    let mut output = Vec::new();
    let mut counter: usize = 0;
    for c in program.chars() {
        output.push((counter, interpret_char(c)));
        counter += 1;
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use cmd::Command;
    use std::vec::Vec;

    #[test]
    fn translates_program_into_map() {
        let program = ">.<";
        let result: Vec<(usize, Command)> = translate(program);
        assert_eq!(3, result.len());
        assert_eq!(Command::ShiftRight, result[0].1);
        assert_eq!(Command::Output, result[1].1);
        assert_eq!(Command::ShiftLeft, result[2].1);
    }

    #[test]
    fn interprets_shift_right_command() {
        let cmd = interpret_char('>');
        assert_eq!(cmd, Command::ShiftRight);
    }

    #[test]
    fn interprets_shift_left_command() {
        let cmd = interpret_char('<');
        assert_eq!(cmd, Command::ShiftLeft);
    }

    #[test]
    fn interprets_increment_command() {
        let cmd = interpret_char('+');
        assert_eq!(cmd, Command::Increment);
    }

    #[test]
    fn interprets_decrement_command() {
        let cmd = interpret_char('-');
        assert_eq!(cmd, Command::Decrement);
    }

    #[test]
    fn interprets_output_command() {
        let cmd = interpret_char('.');
        assert_eq!(cmd, Command::Output);
    }

    #[test]
    fn interprets_input_command() {
        let cmd = interpret_char(',');
        assert_eq!(cmd, Command::Input);
    }

    #[test]
    fn interprets_loop_start_command() {
        let cmd = interpret_char('[');
        assert_eq!(cmd, Command::LoopStart);
    }

    #[test]
    fn interprets_loop_end_command() {
        let cmd = interpret_char(']');
        assert_eq!(cmd, Command::LoopEnd);
    }

}
