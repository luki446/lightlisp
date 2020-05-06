use crate::ast::Environment;

#[derive(Debug, Clone)]
pub enum Cell {
    Number(i64),
    Symbol(String),
    FuncCall(String, Vec<Cell>),
    List(Vec<Cell>),
    BuiltIn(fn(Vec<Cell>) -> Cell),
}

impl Cell 
{
    pub fn eval(&self, env: &Environment) -> Option<Cell>
    {
        match self 
        {
            e @ Cell::Number(_) => {
                return Some(e.clone());
            },
            Cell::Symbol(sym) => {
                return env.find(sym);
            },
            Cell::FuncCall(name, args) => {
                match env.find(name) {
                    Some(x) => match x {
                        Cell::BuiltIn(func) => {
                            return Some(func(args.to_vec()));
                        },
                        _ => {
                            return None;
                        }
                    },
                    None => return None
                }
            },
            Cell::List(cells) => {
                let mut ret: Option<Cell> = None;
                for i in cells {
                    ret = i.eval(env);
                }

                ret
            },
            Cell::BuiltIn(_) => {
                return None;
            }
        }
    }
}
