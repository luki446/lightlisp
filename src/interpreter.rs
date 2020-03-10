use std::collections::HashMap;
use std::fmt;
use std::iter::Iterator;

#[derive(Debug)]
enum Token {
    LeftParen,
    RightParen,
    Number(i32),
    Symbol(String),
}

enum Expr {
    Number(i32),
    Symbol(String),
    List(Vec<Expr>),
    Func(fn(&[Expr]) -> Result<Expr, String>),
}

impl fmt::Display for Expr
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let res = match self {
            Expr::Number(x) => x.to_string(),
            Expr::Symbol(x) => x.to_string(),
            Expr::List(x) => {
                let list: Vec<String> = x.iter().map(|s| s.to_string()).collect();
                format!("({})", list.join(", "))
            },
            Expr::Func(x) => "Function {}".to_string()
        };

        write!(f, "{}", res)
    }
}

pub struct Interpreter {
    ast: bool,
    data: HashMap<String, Expr>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            ast: false,
            data: HashMap::new(),
        }
    }

    pub fn with_ast(mut self) -> Interpreter {
        self.ast = true;
        self
    }

    fn tokenize(src: String) -> Result<Vec<Token>, String> {
        let mut iter = src.chars().peekable();

        let mut res = Vec::new();

        loop {
            match iter.next() {
                Some(x) => match x {
                    '(' => {
                        res.push(Token::LeftParen);
                    }
                    ')' => {
                        res.push(Token::RightParen);
                    }
                    e @ _ if x.is_digit(10) || (x == '-' && iter.peek().unwrap().is_digit(10)) => {
                        let mut foo = String::new();
                        foo.push(e);

                        while iter.peek().unwrap().is_digit(10) {
                            foo.push(iter.next().unwrap())
                        }

                        let parsed;
                        match foo.parse::<i32>() {
                            Ok(x) => parsed = x,
                            Err(x) => {
                                return Err(format!("Error while parsing {} : {}", foo, x));
                            }
                        }

                        res.push(Token::Number(parsed));
                    }
                    e @ _ if x.is_ascii_graphic() => {
                        let mut foo = String::new();
                        foo.push(e);

                        while iter.peek().unwrap().is_ascii_graphic() {
                            foo.push(iter.next().unwrap())
                        }

                        res.push(Token::Symbol(foo));
                    }
                    _ if x.is_whitespace() => (),
                    e @ _ => {
                        return Err(format!("Unexpected token {}", e));
                    }
                },
                None => {
                    break;
                }
            }
        }

        return Ok(res);

        //Ok(vec![Token::LeftParen, Token::Symbol("+".to_string()), Token::Number(123), Token::Number(231), Token::Number(731)])
    }

    fn parse(tokens: Vec<Token>) -> Result<Expr, String>
    {
        
    }

    pub fn interpret(&mut self, src: String) {
        let tokens = Interpreter::tokenize(src).unwrap();

        println!("{:?}", tokens);
    }
}
