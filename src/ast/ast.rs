use crate::ast::Environment;
use std::*;

pub struct ASTree(pub Vec<Cell>);

impl fmt::Debug for ASTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        writeln!(f, "[")?;
        for elem in &self.0 {
            writeln!(f, "{:?}", elem)?;
        }
        writeln!(f, "]")?;

        Ok(())
    }
}

#[derive(Clone)]
pub enum Cell {
    Number(i64),
    Symbol(String),
    FuncCall(String, Vec<Cell>),
    List(Vec<Cell>),
    BuiltIn(fn(&Vec<Cell>, &Environment) -> Cell),
}

impl Cell {
    pub fn eval(&self, env: &Environment) -> Option<Cell> {
        match self {
            e @ Cell::Number(_) => {
                return Some(e.clone());
            }
            Cell::Symbol(sym) => {
                return env.find(sym);
            }
            Cell::FuncCall(name, args) => match env.find(name) {
                Some(x) => match x {
                    Cell::BuiltIn(func) => {
                        return Some(func(args, &env));
                    }
                    _ => {
                        return None;
                    }
                },
                None => return None,
            },
            Cell::List(cells) => {
                let mut ret: Option<Cell> = None;
                for i in cells {
                    ret = i.eval(env);
                }

                ret
            }
            Cell::BuiltIn(_) => {
                return None;
            }
        }
    }

    fn dbg_print(
        &self,
        depth: usize,
        f: &mut std::fmt::Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        write!(f, "{}", "\t".repeat(depth))?;
        match self {
            Cell::Number(num) => {
                writeln!(f, "Number: {}", num)?;
            }
            Cell::Symbol(sym) => {
                writeln!(f, "Symbol: {}", sym)?;
            }
            Cell::FuncCall(name, args) => {
                writeln!(f, "Call: {}", name)?;
                writeln!(f, "{}Args:", "\t".repeat(depth))?;
                for arg in args {
                    arg.dbg_print(depth + 1, f)?;
                }
            }
            Cell::List(list) => {
                writeln!(f, "List:")?;
                for elem in list {
                    elem.dbg_print(depth + 1, f)?;
                }
            }
            Cell::BuiltIn(_) => {
                writeln!(f, "__")?;
            }
        }

        Ok(())
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        match self {
            Cell::Number(num) => {
                write!(f, "{}", num)?;
            },
            _ => {
                write!(f, "__")?;
            } 
        }

        Ok(())
    }
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        self.dbg_print(0, f)?;

        Ok(())
    }
}
