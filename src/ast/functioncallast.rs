use crate::interpreter::ast::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::fmt;
use std::borrow::Borrow;


pub struct FunctionCallAST {
    name: String,
    args: Vec<Rc<dyn ExprAST>>
}

impl FunctionCallAST {
    pub fn new(name: String, args: Vec<Rc<dyn ExprAST>>) -> FunctionCallAST {
        FunctionCallAST {
            name,
            args
        }
    }
}

impl ExprAST for FunctionCallAST {
    fn eval(&self, data: &mut HashMap<String, Rc<dyn ExprAST>>) -> BasicType {
        BasicType::Boolean(true)
    }
    fn display(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result { 
        write!(f, "{}Function {}\n", "\t".repeat(depth),self.name)?;
        write!(f, "{}Args:\n", "\t".repeat(depth))?;
        for i in &self.args {
            i.display(f, depth + 1)?;
        }

        Ok(())
     }
}