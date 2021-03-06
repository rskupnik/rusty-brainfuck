//! Holds the actual `VirtualMachine` that executes the program

use cmd::Command;
use lp::Loop;
use std::collections::HashMap;
use std::vec::Vec;
use interpreter::translate;
use std::io::stdin;
use std::string::String;

/// The `VirtualMachine` is very simple, it only has a 100 bytes of memory
/// and a `memory_ptr` to point to the current memory slot.
pub struct VirtualMachine {
    memory_ptr: u32,
    memory: [u8; 100]
}

impl VirtualMachine {

    /// Constructs a new `VirtualMachine` - the memory is initialized to 0
    /// and the `memory_ptr` is set to 0.
    pub fn new() -> VirtualMachine {
        VirtualMachine { memory_ptr: 0, memory: [0; 100] }
    }

    /// Executes a program in a form of a `&str` and returns the result
    /// in the form of a heap-allocated `String`.
    pub fn execute_program(&mut self, program: &str) -> String {
	let mut output: String = String::new();
	
	let commands: Vec<(usize, Command)> = translate(program);
	let loops: HashMap<usize, Loop> = find_loops(&commands);
	
	let mut loop_stack: Vec<&Loop> = Vec::new();
	let mut program_counter: usize = 0;
	loop {
	    if program_counter >= commands.len() {
		break;
            }

            let &(pos, ref cmd) = &commands[program_counter];
	    if !self.execute_stateless_command(cmd) {
		match cmd {
		    &Command::Output => {
			output.push(self.output() as char);
			//print!("{}", output as char);
		    },
		    &Command::Input => {
			let c = get_input().expect("single char as input");
                        self.input(c as u8);
		    },
		    &Command::LoopStart => {
			let lp = loops.get(&pos).expect("A loop found earlier");
			let output = self.output();
			if output == 0 {
			    program_counter = *lp.start_pos();
			} else {
			    loop_stack.push(lp);
			}
		    },
		    &Command::LoopEnd => {
			let output = self.output();
			if output != 0 {
			    program_counter = {
				let lp: &Loop = &loop_stack[loop_stack.len()-1];
				*lp.start_pos()
			    };
			} else {
			    loop_stack.pop();
			}
		    }
		    _ => ()
		}
	    }

            program_counter += 1;
        }

	output
    }

    fn execute_stateless_command(&mut self, cmd: &Command) -> bool {
        match cmd {
            &Command::ShiftRight => {
		self.shift_right();
		true
	    },
            &Command::ShiftLeft => { 
		self.shift_left();
		true
	    },
            &Command::Increment => { 
		self.increment();
		true
	    },
            &Command::Decrement => { 
		self.decrement();
		true
	    },
            _ => false
        }
    }

    fn increment(&mut self) {
        self.memory[self.memory_ptr as usize] += 1;
        //println!("Memory value at {} increased and is now {}", self.memory_ptr, self.memory[self.memory_ptr as usize]);
    }

    fn decrement(&mut self) {
        self.memory[self.memory_ptr as usize] -=1;
        //println!("Memory value at {} decreased and is now {}", self.memory_ptr, self.memory[self.memory_ptr as usize]);
    }

    fn shift_right(&mut self) {
        if self.memory_ptr != 99 {
            self.memory_ptr += 1;
        }
        //println!("Memory pointer shifted right and is now at {}", self.memory_ptr);
    }

    fn shift_left(&mut self) {
        if self.memory_ptr != 0 {
            self.memory_ptr -= 1;
        }
        //println!("Memory pointer shifted left and is now at {}", self.memory_ptr);
    }

    fn input(&mut self, val: u8) {
        self.memory[self.memory_ptr as usize] = val;
    }

    fn output(&self) -> u8 {
        self.memory[self.memory_ptr as usize]
    }
}

fn find_loops(commands: &Vec<(usize, Command)>) -> HashMap<usize, Loop> { 
    let mut output: HashMap<usize, Loop> = HashMap::new();
    let mut loop_stack: Vec<Loop> = Vec::new();
    for &(pos, ref command) in commands {
	//println!("(pos, command) == {} {:?}", pos, command);
        match command {
	    &Command::LoopStart => {
		loop_stack.push(Loop::new(pos));
            },
	    &Command::LoopEnd => {
		let mut lp = loop_stack.pop().expect("a loop should be on the stack");
                lp.end_pos = pos;
                output.insert(*lp.start_pos(), lp);
		
	    },
	    _ => ()
        }
    }
    output
}

fn get_input() -> Option<char> {
    let mut input = String::new();
    match stdin().read_line(&mut input) {
	Ok(_n) => Some(input.chars().next().unwrap()),
	Err(error) => { 
	    println!("error: {}", error);
	    None
	}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cmd::Command;
    use lp::Loop;
    use std::collections::HashMap;
    use std::vec::Vec;
    use interpreter::translate;

    #[test]
    fn should_execute_hello_world_program() {
	let program = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
	let mut vm = VirtualMachine::new();
	let output = vm.execute_program(program);
        assert_eq!("Hello World!\n", output);
    }

    #[test]
    fn should_find_loops() {
	let program_str = "[>]";
        let program: Vec<(usize, Command)> = translate(program_str);
        let lps: HashMap<usize, Loop> = find_loops(&program);

        assert_eq!(1, lps.len());

	let lp = lps.get(&0).expect("A loop at position 0");
        assert_eq!(0, *lp.start_pos());
        assert_eq!(2, lp.end_pos);
    }

    #[test]
    fn should_find_nested_loops() {
        let program_str = "[>[>[>]<]<]";
        let program: Vec<(usize, Command)> = translate(program_str);
        let lps: HashMap<usize, Loop> = find_loops(&program);

        assert_eq!(3, lps.len());

        let lp = lps.get(&0).expect("A loop at position 0");
        assert_eq!(0, *lp.start_pos());
        assert_eq!(10, lp.end_pos);

        let lp = lps.get(&2).expect("A loop at position 2");
        assert_eq!(2, *lp.start_pos());
        assert_eq!(8, lp.end_pos);

        let lp = lps.get(&4).expect("A loop at position 4");
        assert_eq!(4, *lp.start_pos());
        assert_eq!(6, lp.end_pos);
    }

    #[test]
    fn input_should_set_value_and_output_should_read() {
        let mut vm = VirtualMachine::new();
        vm.input(40);
        let val = vm.output();

        assert_eq!(40, val);
    }

    #[test]
    fn shift_right_should_increase_memory_pointer() {
        let mut vm = VirtualMachine::new();
        vm.shift_right();
        
        assert_eq!(1, vm.memory_ptr);
    }

    #[test]
    fn shift_right_should_not_increase_memory_pointer_above_threshold() {
        let mut vm = VirtualMachine::new();
        vm.memory_ptr = 99;
        vm.shift_right();

        assert_eq!(99, vm.memory_ptr);
    }

    #[test]
    fn shift_left_should_decrease_memory_pointer() {
        let mut vm = VirtualMachine::new();
        vm.memory_ptr = 99;
        vm.shift_left();

        assert_eq!(98, vm.memory_ptr);
    }

    #[test]
    fn shift_left_should_not_decrease_memory_pointer_below_threshold() {
        let mut vm = VirtualMachine::new();
        vm.shift_left();

        assert_eq!(0, vm.memory_ptr);
    }

    #[test]
    fn increment_should_increase_value_at_memory_pointer() {
        let mut vm = VirtualMachine::new();
        vm.increment();
        let val = vm.output();

        assert_eq!(1, val);
    }

    #[test]
    #[should_panic]
    fn increment_over_threshold_should_panic() {
        let mut vm = VirtualMachine::new();
        vm.input(255);
        vm.increment();
    }

    #[test]
    fn decrement_should_decrease_value_at_memory_pointer() {
        let mut vm = VirtualMachine::new();
        vm.input(1);
        vm.decrement();
        let val = vm.output();

        assert_eq!(0, val);
    }

    #[test]
    #[should_panic]
    fn decrement_below_threshold_should_panic() {
        let mut vm = VirtualMachine::new();
        vm.decrement();
    }
}
