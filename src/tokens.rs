#[derive(Debug, PartialEq)]
pub enum Token {
    LeftParen,
    RightParen,
    Number(i64),
    Symbol(String),
}

pub fn tokenize(src: String) -> Result<Vec<Token>, String> {
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
                    match foo.parse::<i64>() {
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
}
