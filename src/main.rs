mod interpreter;
mod common;

use interpreter::interpret;

fn main() {
    let val = interpret(">");
    println!("{:?}", val);
}

