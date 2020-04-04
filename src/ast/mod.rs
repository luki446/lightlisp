pub mod numberast;
pub mod functioncallast;
pub mod symbolast;
use std::rc::Rc;
use std::collections::HashMap;
use std::fmt;

use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

pub enum BasicType{
    Number(i64),
    Boolean(bool)
}

pub trait ExprAST{
    fn eval(&self, data: &mut HashMap<String, Rc<dyn ExprAST>>) -> BasicType;
    fn display(&self, f: &mut fmt::Formatter, depth: usize) -> fmt::Result;
}

pub struct Visitor<'a>(pub &'a Vec<Rc<dyn ExprAST>>);

impl std::fmt::Display for Visitor<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in self.0 {
            i.display(f, 0)?;
        }
        Ok(())
    }
}