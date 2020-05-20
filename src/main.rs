mod ast;
mod interpreter;
mod parser;
mod stdlib;
mod tokens;
mod opt;

use std::fs::File;
use std::io::prelude::*;
use opt::Opt;
use structopt::StructOpt;
use interpreter::*;

fn main() {
    use std::io::{stdin, stdout, Write};

    let opt = Opt::from_args();

    println!("{:#?}", opt);

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
        },
        None => {
            let mut input = String::new();
            loop {
                print!(">>> ");
                stdout().flush().expect("Cannot flush buffer");
                stdin().read_line(&mut input).expect("Error user input");
                if !input.trim().is_empty() {
                    println!("{}", int.interpret(&input));
                }
            }
        }
    }
}
