use crate::ast::cells::Cell;
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
    let mut value: i64 = match args[0] {
        Cell::Number(x) => x,
        _ => 0,
    };

    for i in args.iter().skip(1) {
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
    let mut value: i64;

    match args[0] {
        Cell::Number(num) => {
            value = num;
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
    stdlib_expand!{env,
        "+" => addition,
        "-" => subtraction,
        "*" => multipication,
        "/" => division
    };
}
