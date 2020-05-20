mod ast;
mod interpreter;
mod parser;
mod stdlib;
mod tokens;

use interpreter::*;

fn main() {
    use std::io::{stdin, stdout, Write};

    let mut int = Interpreter::new().with_ast();

    let mut input = String::new();
    loop {
        print!(">>> ");
        let _ = stdout().flush();
        stdin().read_line(&mut input).expect("Error user input");
        if !input.trim().is_empty() {
            int.interpret(&input);
        }
    }
    //int.interpret("(print (+ 1 2 3(* 2 3 4)))".to_string());
}
