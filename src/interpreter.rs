use std::collections::HashMap;

use crate::ast::Environment;
use crate::parser::parse;
use crate::tokens::tokenize;
use crate::ast::ast::Cell;

pub struct Interpreter {
    ast: bool,
    env: Environment,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            ast: false,
            env: Environment::new(),
        }
    }

    pub fn with_ast(mut self) -> Interpreter {
        self.ast = true;
        self
    }

    pub fn interpret(&mut self, src: String) {
        let tokens = tokenize(src).unwrap();
        let expr_tree = parse(&tokens).unwrap();

        if self.ast {
            println!("{:#?}", expr_tree);
        }

        // for i in expr_tree {
        //     i.eval(&mut self.data);
        // }
    }

    // fn eval(&mut self, tree: Vec<Cell>, env: &Environment) -> Option<Cell>{
    //     for i in tree
    //     {
    //         match i 
    //         {
    //             e @ Cell::Number(_) => {
    //                 return Some(e)
    //             },
    //             Symbol(sym) => {
    //                 return env.find(sym);
    //             }
    //             FuncCall(name, args) => {
    //                 return 
    //             }
    //             List(Vec<Cell>),
    //             BuiltIn(fn(Vec<Cell>) -> Cell),
    //         }
    //     }

    //     return None;
    // } 
}
