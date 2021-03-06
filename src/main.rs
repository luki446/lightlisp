mod ast;
mod interpreter;
mod opt;
mod parser;
mod stdlib;
mod tokens;

use interpreter::*;
use opt::Opt;
use std::fs::File;
use std::io::prelude::*;
use structopt::StructOpt;

fn main() {
    use std::io::{stdin, stdout};

    let opt = Opt::from_args();

    let mut int = Interpreter::new();

    int.ast = opt.ast;

    match opt.file_name {
        Some(name) => {
            let mut file = match File::open(&name) {
                Ok(file) => file,
                Err(error) => {
                    panic!("Error while opening file: {}", error);
                }
            };

            let mut content = String::new();

            match file.read_to_string(&mut content) {
                Ok(_) => (),
                Err(error) => {
                    panic!("Error while reading from file: {}", error);
                }
            }

            int.interpret(&content);
        }
        None => {
            let mut input = String::new();
            loop {
                print!(">>> ");
                stdout().flush().expect("Cannot flush buffer");
                stdin().read_line(&mut input).expect("Error user input");
                if !input.trim().is_empty() {
                    println!("{}", int.interpret(&input));
                }
                input.clear();
            }
        }
    }
}
