use crate::tokens::*;

use crate::ast::ast::ASTree;
use crate::ast::ast::Cell;

pub fn parse(tokens: &Vec<Token>) -> Result<ASTree, String> {
    let mut iter = tokens.iter().peekable();
    let mut exprs: Vec<Cell> = Vec::new();
    while iter.peek().is_some() {
        let token = iter.next().unwrap();
        if *token == Token::LeftParen {
            exprs.push(parse_expr(&mut iter)?);
        } else {
            return Err(format!("Unexpected token {:?}", *token));
        }
    }

    Ok(ASTree(exprs))
}

fn parse_expr(
    tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
) -> Result<Cell, String> {
    match tokens.peek() {
        Some(x) => match x {
            Token::Symbol(y) => {
                tokens.next();
                let mut args: Vec<Cell> = Vec::new();
                while tokens.peek() != Some(&&Token::RightParen) {
                    match tokens.peek().unwrap() {
                        Token::LeftParen => {
                            tokens.next();
                            args.push(parse_expr(tokens)?);
                        }
                        Token::Number(num) => {
                            args.push(Cell::Number(*num));
                            tokens.next();
                            //println!("{:?}", num);
                        }
                        Token::Symbol(sym) => {
                            args.push(Cell::Symbol(sym.to_string()));
                            tokens.next();
                            //println!("{:?}", sym);
                        }
                        _ => (),
                    }
                }
                tokens.next();
                return Ok(Cell::FuncCall(y.to_string(), args));
            }
            e @ _ => {
                return Err(format!("Unexpected token {:?} at parsing expressions", e));
            }
        },
        None => {
            return Err("Unexpected end of tokens in middle of expression".to_string());
        }
    }
}
