use crate::interpreter::ast::*;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

pub struct SymbolAST {
    symbol: String,
}

impl SymbolAST {
    pub fn new(symbol: String) -> SymbolAST {
        SymbolAST { symbol }
    }
}

impl ExprAST for SymbolAST {
    fn eval(&self, data: &mut std::collections::HashMap<String, Rc<dyn ExprAST>>) -> BasicType {
        BasicType::Number(-1)
    }
    fn display(&self, f: &mut std::fmt::Formatter, depth: usize) -> fmt::Result {
        write!(f, "{}{} <-\n", "\t".repeat(depth), self.symbol)
    }
}
