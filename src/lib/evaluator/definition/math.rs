use crate::lib::evaluator::definition::prelude::*;

pub struct Math;

impl Module for Math {
    fn load() -> Vec<Definition> {
        vec![
            Definition::new(add_sig(), handler!(add_handler)),
            Definition::new(sub_sig(), handler!(sub_handler)),
            Definition::new(mul_sig(), handler!(mul_handler)),
            Definition::new(div_sig(), handler!(div_handler)),
            Definition::new(mod_sig(), handler!(mod_handler)),
            Definition::new(pow_sig(), handler!(pow_handler)),
        ]
    }
}

fn add_sig() -> Token {
    Token::Lst(vec![
        Token::BinaryOp("+".to_owned()),
        Token::Var("Definition".to_owned()),
        Token::Var("Result".to_owned()),
    ])
}

fn sub_sig() -> Token {
    Token::Lst(vec![
        Token::BinaryOp("-".to_owned()),
        Token::Var("Definition".to_owned()),
        Token::Var("Result".to_owned()),
    ])
}

fn mul_sig() -> Token {
    Token::Lst(vec![
        Token::BinaryOp("*".to_owned()),
        Token::Var("Definition".to_owned()),
        Token::Var("Result".to_owned()),
    ])
}

fn div_sig() -> Token {
    Token::Lst(vec![
        Token::BinaryOp("/".to_owned()),
        Token::Var("Definition".to_owned()),
        Token::Var("Result".to_owned()),
    ])
}

fn mod_sig() -> Token {
    Token::Lst(vec![
        Token::BinaryOp("%".to_owned()),
        Token::Var("Definition".to_owned()),
        Token::Var("Result".to_owned()),
    ])
}

fn pow_sig() -> Token {
    Token::Lst(vec![
        Token::BinaryOp("**".to_owned()),
        Token::Var("Definition".to_owned()),
        Token::Var("Result".to_owned()),
    ])
}

fn add_handler(_: &mut EvalState, token: Token) -> EvalResult {
    if let Token::Lst(lst) = token {
        if let [_, Token::Number(a), Token::Number(b)] = &*lst {
            return Ok(Token::Number(a + b));
        }
    }

    Ok(Token::Unknown)
}

fn sub_handler(_: &mut EvalState, token: Token) -> EvalResult {
    if let Token::Lst(lst) = token {
        if let [_, Token::Number(a), Token::Number(b)] = &*lst {
            return Ok(Token::Number(a - b));
        }
    }

    Ok(Token::Unknown)
}

fn mul_handler(_: &mut EvalState, token: Token) -> EvalResult {
    if let Token::Lst(lst) = token {
        if let [_, Token::Number(a), Token::Number(b)] = &*lst {
            return Ok(Token::Number(a * b));
        }
    }

    Ok(Token::Unknown)
}

fn div_handler(_: &mut EvalState, token: Token) -> EvalResult {
    if let Token::Lst(lst) = token {
        if let [_, Token::Number(a), Token::Number(b)] = &*lst {
            return Ok(Token::Number(a / b));
        }
    }

    Ok(Token::Unknown)
}

fn mod_handler(_: &mut EvalState, token: Token) -> EvalResult {
    if let Token::Lst(lst) = token {
        if let [_, Token::Number(a), Token::Number(b)] = &*lst {
            return Ok(Token::Number(a % b));
        }
    }

    Ok(Token::Unknown)
}

fn pow_handler(_: &mut EvalState, token: Token) -> EvalResult {
    if let Token::Lst(lst) = token {
        if let [_, Token::Number(a), Token::Number(b)] = &*lst {
            return Ok(Token::Number(f32::powf(*a, *b)));
        }
    }

    Ok(Token::Unknown)
}
