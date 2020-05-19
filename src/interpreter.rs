use crate::ast::Environment;
use crate::parser::parse;
use crate::stdlib::io::add_basic_io;
use crate::stdlib::math::add_basic_math;
use crate::tokens::tokenize;

pub struct Interpreter {
    ast: bool,
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

    pub fn with_ast(mut self) -> Interpreter {
        self.ast = true;
        self
    }

    pub fn interpret(&mut self, src: String) {
        let tokens = tokenize(src).unwrap();
        let expr_tree = parse(&tokens).unwrap();

        if self.ast {
            println!("{:?}", expr_tree);
            //println!("{}", "=".repeat(60));
        }

        for i in expr_tree.0 {
            i.eval(&mut self.env);
        }
    }
}
