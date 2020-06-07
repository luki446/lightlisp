use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "llisp",
    about = "Simple Light Lisp interpreter",
    author = "made by luki446<luki446@gmail.com>",
)]
pub struct Opt {
    /// Activate Abstract Syntax Tree printing
    #[structopt(short, long)]
    pub ast: bool,

    /// File name to execute
    #[structopt(name = "FILE")]
    pub file_name: Option<String>,
}
