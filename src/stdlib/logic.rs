use crate::ast::cells::Cell;
use crate::ast::Environment;

pub fn if_fn(args: &Vec<Cell>, env: &Environment) -> Cell {
    if args.len() > 1 {
        let logic_value = args[0].eval(&env).unwrap();
        if logic_value != Cell::Nil && logic_value != Cell::Bool(false) {
            return args[1].eval(&env).unwrap();
        } else if args.len() > 2 {
            return args[2].eval(&env).unwrap();
        }
    }

    Cell::Nil
}

pub fn equal_fn(args: &Vec<Cell>, env: &Environment) -> Cell {
    if args.len() > 1 {
        let first = args[0].eval(&env).unwrap();
        return Cell::Bool(
            !args
                .iter()
                .skip(1)
                .find(move |item| **item == first)
                .is_none(),
        );
    }

    Cell::Nil
}

pub fn not_equal_fn(args: &Vec<Cell>, env: &Environment) -> Cell {
    if args.len() > 1 {
        let first = args[0].eval(&env).unwrap();
        return Cell::Bool(
            !args
                .iter()
                .skip(1)
                .find(move |item| item.eval(env).unwrap() == first)
                .is_some(),
        );
    }

    Cell::Nil
}

pub fn lesser_fn(args: &Vec<Cell>, env: &Environment) -> Cell {
    if args.len() > 1 {
        for i in 0..(args.len() - 1) {
            if !(args[i].eval(env).unwrap() < args[i + 1].eval(env).unwrap()) {
                return Cell::Bool(false);
            }
        }
        return Cell::Bool(true);
    }

    return Cell::Nil;
}

pub fn lesser_equal_fn(args: &Vec<Cell>, env: &Environment) -> Cell {
    if args.len() > 1 {
        for i in 0..(args.len() - 1) {
            if !(args[i].eval(env).unwrap() <= args[i + 1].eval(env).unwrap()) {
                return Cell::Bool(false);
            }
        }
        return Cell::Bool(true);
    }

    return Cell::Nil;
}

pub fn greater_fn(args: &Vec<Cell>, env: &Environment) -> Cell {
    if args.len() > 1 {
        for i in 0..(args.len() - 1) {
            if !(args[i].eval(env).unwrap() > args[i + 1].eval(env).unwrap()) {
                return Cell::Bool(false);
            }
        }
        return Cell::Bool(true);
    }

    return Cell::Nil;
}

pub fn greater_equal_fn(args: &Vec<Cell>, env: &Environment) -> Cell {
    if args.len() > 1 {
        for i in 0..(args.len() - 1) {
            if !(args[i].eval(env).unwrap() >= args[i + 1].eval(env).unwrap()) {
                return Cell::Bool(false);
            }
        }
        return Cell::Bool(true);
    }

    return Cell::Nil;
}

pub fn and_fn(args: &Vec<Cell>, env: &Environment) -> Cell {
    if args.len() > 1 {
        return Cell::Bool(
            args.iter()
                .map(|item| item.eval(env).unwrap())
                .all(|item| item != Cell::Bool(false) && item != Cell::Nil),
        );
    }

    return Cell::Nil;
}

pub fn or_fn(args: &Vec<Cell>, env: &Environment) -> Cell {
    if args.len() > 1 {
        return Cell::Bool(
            args.iter()
                .map(|item| item.eval(env).unwrap())
                .any(|item| item != Cell::Bool(false) && item != Cell::Nil),
        );
    }

    return Cell::Nil;
}

pub fn add_basic_logic(env: &mut Environment) {
    stdlib_expand! {env,
        "if" => if_fn,
        "=" => equal_fn,
        "<>" => not_equal_fn,
        "<" => lesser_fn,
        "<=" => lesser_equal_fn,
        ">" => greater_fn,
        ">=" => greater_equal_fn,
        "&&" => and_fn,
        "||" => or_fn
    };
}
