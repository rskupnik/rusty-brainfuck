use common::Command;
use common::Loop;
use std::collections::HashMap;
use std::vec::Vec;
use interpreter::translate;

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

pub struct VirtualMachine {
    memory_ptr: u32,
    memory: [u8; 100]
}

impl VirtualMachine {

    pub fn new() -> VirtualMachine {
        VirtualMachine { memory_ptr: 0, memory: [0; 100] }
    }

    pub fn execute_program(&mut self, program: &str) {
	let commands: Vec<(usize, Command)> = translate(program);
	let loops: HashMap<usize, Loop> = find_loops(&commands);

	let mut program_counter: usize = 0;
	loop {
	    if program_counter >= commands.len() {
		break;
            }

            let cmd = &commands[program_counter];
	    

            program_counter += 1;
        }
    }

    pub fn execute_command(&mut self, cmd: &Command) -> bool {
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

    pub fn increment(&mut self) {
        self.memory[self.memory_ptr as usize] += 1;
        println!("Memory value at {} increased and is now {}", self.memory_ptr, self.memory[self.memory_ptr as usize]);
    }

    pub fn decrement(&mut self) {
        self.memory[self.memory_ptr as usize] -=1;
        println!("Memory value at {} decreased and is now {}", self.memory_ptr, self.memory[self.memory_ptr as usize]);
    }

    pub fn shift_right(&mut self) {
        if self.memory_ptr != 99 {
            self.memory_ptr += 1;
        }
        println!("Memory pointer shifted right and is now at {}", self.memory_ptr);
    }

    pub fn shift_left(&mut self) {
        if self.memory_ptr != 0 {
            self.memory_ptr -= 1;
        }
        println!("Memory pointer shifted left and is now at {}", self.memory_ptr);
    }

    pub fn input(&mut self, val: u8) {
        self.memory[self.memory_ptr as usize] = val;
    }

    pub fn output(&self) -> u8 {
        self.memory[self.memory_ptr as usize]
    }
    
    pub fn memory_ptr(&self) -> &u32 {
        &self.memory_ptr
    }

    pub fn set_memory_ptr(&mut self, val: u32) {
        self.memory_ptr = val;
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use common::Command;
    use common::Loop;
    use std::collections::HashMap;
    use std::vec::Vec;
    use interpreter::translate;

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
    fn set_memory_ptr_should_set_memory_pointer() {
        let mut vm = VirtualMachine::new();
        vm.set_memory_ptr(20);
        
        assert_eq!(20, *vm.memory_ptr());
    }

    #[test]
    fn shift_right_should_increase_memory_pointer() {
        let mut vm = VirtualMachine::new();
        vm.shift_right();
        
        assert_eq!(1, *vm.memory_ptr());
    }

    #[test]
    fn shift_right_should_not_increase_memory_pointer_above_threshold() {
        let mut vm = VirtualMachine::new();
        vm.set_memory_ptr(99);
        vm.shift_right();

        assert_eq!(99, *vm.memory_ptr());
    }

    #[test]
    fn shift_left_should_decrease_memory_pointer() {
        let mut vm = VirtualMachine::new();
        vm.set_memory_ptr(99);
        vm.shift_left();

        assert_eq!(98, *vm.memory_ptr());
    }

    #[test]
    fn shift_left_should_not_decrease_memory_pointer_below_threshold() {
        let mut vm = VirtualMachine::new();
        vm.shift_left();

        assert_eq!(0, *vm.memory_ptr());
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
