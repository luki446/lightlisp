use std::rc::Rc;

use crate::interpreter::tokens::*;

use crate::interpreter::ast::*;
mod ast;
use ast::numberast::*;
use ast::functioncallast::*;
use ast::symbolast::*;


pub fn parse(tokens: &Vec<Token>) -> Result<Vec<Rc<dyn ExprAST>>, String> {
    let mut iter = tokens.iter().peekable();
    let mut exprs: Vec<Rc<dyn ExprAST>> = Vec::new();
    while iter.peek().is_some(){
        let token = iter.next().unwrap();
        if *token == Token::LeftParen {
            exprs.push(parse_expr(&mut iter)?);
        } else {
            return Err(format!("Unexpected token {:?}", *token))
        }
    }

    Ok(exprs)
}

fn parse_expr(tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>) -> Result<Rc<dyn ExprAST>, String> {
    match tokens.peek()
    {
        Some(x) => match x {
            Token::Symbol(y) => {
                tokens.next();
                let mut args: Vec<Rc<dyn ExprAST>> = Vec::new();
                while tokens.peek() != Some(&&Token::RightParen) {
                    match tokens.peek().unwrap() {
                        Token::LeftParen => {
                            tokens.next();
                            args.push(parse_expr(tokens)?);
                        },
                        Token::Number(num) => {
                            args.push(Rc::new(NumberAST::new(*num)));
                            tokens.next();
                            //println!("{:?}", num);
                        },
                        Token::Symbol(sym) => {
                            args.push(Rc::new(SymbolAST::new(sym.to_string())));
                            tokens.next();
                            //println!("{:?}", sym);
                        },
                        _ => ()
                    }
                }
                tokens.next();
                return Ok(Rc::new(FunctionCallAST::new(y.to_string(), args)))

            }, 
            e @ _ => {
                return Err(format!("Unexpected token {:?} at parsing expressions", e));
            }
        },
        None => {
            return Err("Unexpected end of tokens in middle of expression".to_string());
        }
    }
}