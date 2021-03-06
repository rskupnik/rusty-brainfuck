mod interpreter;
mod cmd;
mod lp;
mod vm;

use vm::VirtualMachine;

fn main() {
    let mut vm = VirtualMachine::new();
    let program = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
    let output = vm.execute_program(program);
    print!("{}", output);
}

