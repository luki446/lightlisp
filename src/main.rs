mod interpreter;
mod ast;
mod tokens;
mod parser;


use ast::*;
use interpreter::*;

fn main() {
    let mut int = Interpreter::new().with_ast();

    int.interpret("(print (+ 1 2 3))(- 3 2 (* 2 1))".to_string());
}
