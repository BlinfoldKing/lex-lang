use crate::lib::evaluator::definition::prelude::*;
use crate::lib::printer::print_token;

pub struct Misc;

impl Module for Misc {
    fn load() -> Vec<Definition> {
        vec![
            Definition::new(print_sig(), handler!(print_handler)),
            Definition::new(return_sig(), handler!(return_handler)),
        ]
    }
}

fn print_sig() -> Token {
    Token::Lst(vec![
        Token::Keyword(".print".to_owned()),
        Token::Var("Any".to_owned()),
    ])
}

fn return_sig() -> Token {
    Token::Lst(vec![
        Token::Keyword(".return".to_owned()),
        Token::Var("Any".to_owned()),
    ])
}

fn print_handler(_: &mut EvalState, token: Token) -> EvalResult {
    if let Token::Lst(lst) = token {
        if let [_, t] = &*lst.clone() {
            print_token(t.clone());
            return Ok(Token::Bool(true));
        }
    }

    Ok(Token::Bool(false))
}

fn return_handler(state: &mut EvalState, token: Token) -> EvalResult {
    if let Token::Lst(lst) = token {
        if let [_, t] = &*lst.clone() {
            state.return_value = Some(t.clone());
            return Ok(t.clone());
        }
    }

    Ok(Token::Bool(false))
}
