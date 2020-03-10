mod interpreter;

use interpreter::*;

fn main() {
    let mut int = Interpreter::new().with_ast();

    int.interpret("(+ 123 234 731 (- 12 16))".to_string());
}
