use crate::lib::evaluator::definition::prelude::*;

use std::collections::HashMap;

pub struct Def;

impl Module for Def {
    fn load() -> Vec<Definition> {
        vec![
            Definition::new(def_sig(), handler!(def_handler)),
            Definition::new(dec_sig(), handler!(dec_handler)),
        ]
    }
}

fn def_sig() -> Token {
    Token::Lst(vec![
        Token::Keyword(".def".to_owned()),
        Token::Var("Definition".to_owned()),
        Token::Var("Result".to_owned()),
    ])
}

fn dec_sig() -> Token {
    Token::Lst(vec![
        Token::Keyword(".dec".to_owned()),
        Token::Var("Declaration".to_owned()),
    ])
}

fn match_variable(sig: Token, value: Token) -> HashMap<String, Token> {
    let mut hm: HashMap<String, Token> = HashMap::new();
    match (sig, value) {
        (Token::Var(s), v) => {
            hm.insert(s, v);
        }
        (Token::Lst(nsig), Token::Lst(nval)) => {
            for i in 0..nsig.len() {
                let nhm = match_variable(nsig[i].clone(), nval[i].clone());
                hm.extend(nhm);
            }
        }
        _ => (),
    }

    hm
}

fn replace_variable(token: Token, variables: HashMap<String, Token>) -> Token {
    match token {
        Token::Var(s) => variables.get(&s).unwrap().clone(),
        Token::Lst(lst) => {
            let ret: Vec<Token> = lst
                .into_iter()
                .map(move |token| replace_variable(token, variables.clone()))
                .collect();

            Token::Lst(ret)
        }
        t => t,
    }
}

fn def_handler(state: &mut EvalState, token: Token) -> EvalResult {
    if let Token::Lst(lst) = token {
        if let [_, a, b] = &*lst {
            let param = a.clone();
            let result = b.clone();
            state.definition.push(Definition::new(
                param.clone(),
                handler!(move |_, token| {
                    let variables = match_variable(param.clone(), token);
                    let ret = replace_variable(result.clone(), variables);
                    Ok(ret)
                }),
            ));

            return Ok(Token::Bool(true));
        }
    }

    Ok(Token::Bool(false))
}

fn dec_handler(state: &mut EvalState, token: Token) -> EvalResult {
    if let Token::Lst(lst) = token {
        if let [_, a] = &*lst {
            let param = a.clone();
            state.definition.push(Definition::new(
                param,
                handler!(move |_, _| { Ok(Token::Bool(true)) }),
            ));

            return Ok(Token::Bool(true));
        }
    }

    Ok(Token::Bool(false))
}
