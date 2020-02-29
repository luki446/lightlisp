#[derive(Debug)]
enum Token {
    LeftParen,
    RightParen,
    Number(i32),
    Symbol(String),
}

#[derive(Copy, Clone)]
pub struct Interpreter {
    ast: bool,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter { ast: false }
    }

    pub fn with_ast(&self) -> Interpreter {
        let mut res = self.clone();
        res.ast = true;
        res
    }

    fn tokenize(self, src: String) -> Result<Vec<Token>, String> {
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

                        let mut parsed = 0;
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

    pub fn interpret(&mut self, src: String) {
        let tokens = self.tokenize(src).unwrap();

        println!("{:?}", tokens);
    }
}
