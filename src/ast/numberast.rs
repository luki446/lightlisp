use crate::interpreter::ast::*;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

pub struct NumberAST {
    value: i64,
}

impl NumberAST {
    pub fn new(value: i64) -> NumberAST {
        NumberAST { value }
    }
}

impl ExprAST for NumberAST {
    fn eval(&self, data: &mut HashMap<String, Rc<dyn ExprAST>>) -> BasicType {
        BasicType::Number(self.value)
    }
    fn display(&self, f: &mut fmt::Formatter, depth: usize) -> fmt::Result {
        write!(f, "{}{}\n", "\t".repeat(depth), self.value)
    }
}
