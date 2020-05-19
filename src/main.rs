mod ast;
mod interpreter;
mod parser;
mod stdlib;
mod tokens;

use interpreter::*;

fn main() {
    let mut int = Interpreter::new().with_ast();

    int.interpret("(print (+ 1 2 3(* 2 3 4)))".to_string());
}
