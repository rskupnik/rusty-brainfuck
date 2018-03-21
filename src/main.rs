mod interpreter;
mod common;
mod vm;

use interpreter::interpret;
use vm::VirtualMachine;

fn main() {
    let val = interpret(">");
    println!("{:?}", val);

    let vm = VirtualMachine::new();
    let mem_ptr = vm.memory_ptr();
    println!("{}", mem_ptr);
}

