use crate::ast::ast::Cell;
use crate::ast::Environment;

pub fn addition(args: &Vec<Cell>, env: &Environment) -> Cell {
    let mut value: i64 = 0;
    for i in args {
        let val = i.eval(&env);
        if val.is_some() {
            match val.unwrap() {
                Cell::Number(x) => {
                    value += x;
                }
                _ => (),
            }
        }
    }

    Cell::Number(value)
}

pub fn subtraction(args: &Vec<Cell>, env: &Environment) -> Cell {
    let mut value: i64 = 0;
    for i in args {
        let val = i.eval(&env);
        if val.is_some() {
            match val.unwrap() {
                Cell::Number(x) => {
                    value -= x;
                }
                _ => (),
            }
        }
    }

    Cell::Number(value)
}

pub fn multipication(args: &Vec<Cell>, env: &Environment) -> Cell {
    let mut value: i64 = 1;
    for i in args {
        let val = i.eval(&env);
        if val.is_some() {
            match val.unwrap() {
                Cell::Number(x) => {
                    value *= x;
                }
                _ => (),
            }
        }
    }

    Cell::Number(value)
}

pub fn division(args: &Vec<Cell>, env: &Environment) -> Cell {
    let mut value: i64 = 0;

    match args[0] {
        Cell::Number(num) => {
            let mut value = num;
        }
        _ => {
            return Cell::Number(0);
        }
    }

    for i in args.iter().skip(1) {
        let val = i.eval(&env);
        if val.is_some() {
            match val.unwrap() {
                Cell::Number(x) => {
                    if x != 0 {
                        value /= x;
                    }
                }
                _ => (),
            }
        }
    }

    Cell::Number(value)
}

pub fn add_basic_math(env: &mut Environment) {
    env.data.insert("+".to_string(), Cell::BuiltIn(addition));
    env.data.insert("-".to_string(), Cell::BuiltIn(subtraction));
    env.data
        .insert("*".to_string(), Cell::BuiltIn(multipication));
    env.data.insert("/".to_string(), Cell::BuiltIn(division));
}
