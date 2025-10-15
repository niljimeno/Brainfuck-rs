use std::env::args;
mod interpreter;

fn main() {
    let arguments = args().peekable();

    if arguments.len() < 2 {
        println!();
        println!("=== Cool Char Lang RS ===");
        println!("Usage: ccl-rs <program name>");
        println!();
    }

    for arg in arguments.skip(1) {
        let program = std::fs::read_to_string(arg).unwrap();
        interpreter::run(program.as_str());
    }
}
