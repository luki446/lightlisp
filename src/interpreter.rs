use std::collections::HashMap;
use std::rc::Rc;

#[path="tokens.rs"] mod tokens;
use tokens::*;

#[path="parser.rs"] mod parser;
use parser::*;

#[path = "ast/mod.rs"] mod ast;
use ast::*;

pub struct Interpreter {
    ast: bool,
    data: HashMap<String, Rc<dyn ExprAST>>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            ast: false,
            data: HashMap::new(),
        }
    }

    pub fn with_ast(mut self) -> Interpreter {
        self.ast = true;
        self
    }

    pub fn interpret(&mut self, src: String) {
        let tokens = tokenize(src).unwrap();
        let expr_tree = parse(&tokens).unwrap();

        let vis = Visitor(&expr_tree);
        println!("{}", vis);

        // for i in expr_tree {
        //     i.eval(&mut self.data);
        // }
    }
}
