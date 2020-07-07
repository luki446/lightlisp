use crate::ast::cells::Cell;
use crate::ast::Environment;

pub fn print_func(args: &Vec<Cell>, env: &Environment) -> Cell {
    let mut counter = 0;
    for i in args {
        counter += print_intern(i, &env);
    }

    Cell::Number(counter)
}

fn print_intern(to_print: &Cell, env: &Environment) -> i64 {
    let mut counter = 0;

    match to_print {
        Cell::Number(num) => {
            println!("{}", num);
            counter = 1;
        }
        Cell::Nil => {
            println!("nil");
            counter = 1;
        }
        Cell::Bool(x) => {
            println!("{}", x);
            counter = 1;
        }
        Cell::Symbol(sym) => {
            counter = match env.find(&sym) {
                Some(x) => print_intern(&x, &env),
                None => {
                    eprintln!("Error: Symbol {} not found", sym);
                    0
                }
            }
        }
        Cell::List(list) => {
            for i in list {
                counter += print_intern(i, &env);
            }
        }
        Cell::FuncCall(name, args) => match env.find(&name) {
            Some(smth) => match smth {
                Cell::BuiltIn(func) => {
                    counter = print_intern(&func(&args, &env), &env);
                }
                _ => {
                    eprintln!("Error: {} func cannot be found", name);
                }
            },
            None => {
                eprintln!("Error: {} function cannot be found", name);
            }
        },
        Cell::BuiltIn(_) => {
            eprintln!("Error: Built-in function cannot be printed");
        }
    }

    counter
}

pub fn add_basic_io(env: &mut Environment) {
    env.data
        .insert("print".to_string(), Cell::BuiltIn(print_func));
}
