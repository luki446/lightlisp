pub mod cells;

use cells::Cell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Environment {
    parent: Option<Rc<Environment>>,
    pub data: HashMap<String, Cell>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            parent: None,
            data: HashMap::<String, Cell>::new(),
        }
    }

    pub fn find(&self, symbol: &String) -> Option<Cell> {
        match self.data.get(symbol) {
            Some(x) => Some(x.clone()),
            None => match &self.parent {
                Some(y) => y.find(symbol),
                None => None,
            },
        }
    }
}
