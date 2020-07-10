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

pub fn add_basic_logic(env: &mut Environment) {
    env.data.insert("if".to_string(), Cell::BuiltIn(if_fn));
}
