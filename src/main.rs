mod interpreter;
mod common;
mod vm;

use vm::VirtualMachine;

fn main() {
    let mut vm = VirtualMachine::new();
    let program = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
    vm.execute_program(program);
}

