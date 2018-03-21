use common::Command;

pub struct VirtualMachine {
    memory_ptr: u32,
    memory: [u8; 100]
}

impl VirtualMachine {

    pub fn new() -> VirtualMachine {
        VirtualMachine { memory_ptr: 0, memory: [0; 100] }
    }

    pub fn execute_command(&mut self, cmd: &Command) -> u8 {
        match cmd {
            &Command::ShiftRight => {
                if self.memory_ptr != 99 {
                    self.memory_ptr += 1;
                }
                println!("Memory pointer shifted right and is now at {}", self.memory_ptr);
                0
            }
            &Command::ShiftLeft => {
                if self.memory_ptr != 0 {
                    self.memory_ptr -= 1;
                }
                println!("Memory pointer shifted left and is now at {}", self.memory_ptr);
                0
            }
            &Command::Increment => {
                self.memory[self.memory_ptr as usize] += 1;
                println!("Memory value at {} increased and is now {}", self.memory_ptr, self.memory[self.memory_ptr as usize]);
                0
            }
            &Command::Decrement => {
                self.memory[self.memory_ptr as usize] -= 1;
                println!("Memory value at {} decreased and is now {}", self.memory_ptr, self.memory[self.memory_ptr as usize]);
                0
            }
            _ => {
                println!("{:?}", cmd);
                0
            }
        }
    }

    pub fn execute_input_command(&mut self, val: u8) {
        self.memory[self.memory_ptr as usize] = val;
    }

    pub fn execute_output_command(&self) -> u8 {
        self.memory[self.memory_ptr as usize]
    }
    
    pub fn memory_ptr(&self) -> &u32 {
        &self.memory_ptr
    }

    pub fn set_memory_ptr(&mut self, val: u32) {
        self.memory_ptr = val;
    }
}
