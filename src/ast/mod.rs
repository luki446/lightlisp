pub mod ast;

use std::rc::Rc;

pub struct Environment
{
    parent: Option<Rc<Environment>>
}

impl Environment
{
    pub fn new() -> Environment
    {
        Environment {
            parent: None
        }
    }
}