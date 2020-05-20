use crate::ast::ast::Cell;
use crate::ast::Environment;
use crate::parser::parse;
use crate::stdlib::io::add_basic_io;
use crate::stdlib::math::add_basic_math;
use crate::tokens::tokenize;

pub struct Interpreter {
    pub ast: bool,
    env: Environment,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut inter = Interpreter {
            ast: false,
            env: Environment::new(),
        };
        add_basic_io(&mut inter.env);
        add_basic_math(&mut inter.env);

        inter
    }

    pub fn interpret(&mut self, src: &String) -> Cell {
        let mut result = Cell::Number(0);

        let tokens = tokenize(src).unwrap();
        let expr_tree = parse(&tokens).unwrap();

        if self.ast {
            println!("{:?}", expr_tree);
            //println!("{}", "=".repeat(60));
        }

        for i in expr_tree.0 {
            result = i.eval(&mut self.env).unwrap();
        }

        result
    }
}
