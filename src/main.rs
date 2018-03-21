mod interpreter;
mod common;
mod vm;

use interpreter::interpret;
use vm::VirtualMachine;

fn main() {
    let cmd = interpret(">");
    println!("{:?}", cmd);

    let mut vm = VirtualMachine::new();
    {
        let mem_ptr = vm.memory_ptr();
        println!("{}", mem_ptr);
    }

    vm.execute_command(&cmd);

    let cmd = interpret("+");
    vm.execute_command(&cmd);

    {
        let val = vm.execute_output_command();
        println!("{}", val);
    }

    vm.execute_input_command(20);
    {
       let val = vm.execute_output_command();
       println!("{}", val);
    }
}

