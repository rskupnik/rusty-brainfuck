mod interpreter;
mod common;
mod vm;

use interpreter::interpret;
use vm::VirtualMachine;

fn main() {
    let cmd = interpret(">", 0);
    println!("{:?}", cmd);

    let mut vm = VirtualMachine::new();
    {
        let mem_ptr = vm.memory_ptr();
        println!("{}", mem_ptr);
    }

    vm.execute_command(&cmd);

    let cmd = interpret("+", 0);
    vm.execute_command(&cmd);

    let cmd = interpret(".", 0);
    {
        let val = vm.execute_command(&cmd);
        println!("{}", val);
    }

    let cmd = interpret(",", 20);
    vm.execute_command(&cmd);
    {
       let cmd = interpret(".", 0);
       let val = vm.execute_command(&cmd);
       println!("{}", val);
    }
}

