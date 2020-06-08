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

#[allow(dead_code)]
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
            e @ Cell::Number(_) => Some(e.clone()),
            Cell::Symbol(sym) => env.find(sym),
            Cell::FuncCall(name, args) => match env.find(name) {
                Some(x) => match x {
                    Cell::BuiltIn(func) => Some(func(args, &env)),
                    _ => None,
                },
                None => None,
            },
            Cell::List(cells) => {
                let mut ret: Option<Cell> = None;
                for i in cells {
                    ret = i.eval(env);
                }

                ret
            }
            Cell::BuiltIn(_) => None,
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
            }
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

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Cell::Number(num) => {
                if let Cell::Number(num2) = other {
                    num2 == num
                } else {
                    false
                }
            }
            Cell::Symbol(sym) => {
                if let Cell::Symbol(sym2) = other {
                    sym == sym2
                } else {
                    false
                }
            }
            Cell::FuncCall(name, args) => {
                if let Cell::FuncCall(name2, args2) = other {
                    name == name2 && args == args2
                } else {
                    false
                }
            }
            Cell::List(list) => {
                if let Cell::List(list2) = other {
                    list == list2
                } else {
                    false
                }
            }
            Cell::BuiltIn(_) => false,
        }
    }
}
