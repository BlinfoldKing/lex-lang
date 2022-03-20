pub mod definition;

use definition::Definition;
use definition::Module;

use crate::lib::error::EvalError;
use crate::lib::token::Token;

use std::collections::HashMap;

pub type EvalResult = Result<Token, EvalError>;

#[derive(Clone)]
pub struct EvalState {
    definition: Vec<Definition>,
    variables: HashMap<String, Token>,
    return_value: Option<Token>,
}

impl EvalState {
    pub fn new() -> Self {
        EvalState {
            definition: vec![],
            variables: HashMap::new(),
            return_value: None,
        }
    }
}

#[derive(Clone)]
pub struct Evaluator {
    curr_state: EvalState,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            curr_state: EvalState::new(),
        }
    }

    pub fn load<T: Module>(&mut self, _: T) {
        for item in T::load() {
            self.curr_state.definition.push(item);
        }
    }

    fn eval_lst(&mut self, input: Vec<Token>) -> EvalResult {
        let lst = input
            .into_iter()
            .map(|token| self.eval_token(token).unwrap())
            .collect();

        let token = Token::Lst(lst);

        for def in self.curr_state.definition.clone() {
            if self.match_token(def.signature.clone(), token.clone()) {
                let res = def.run(&mut self.curr_state, token.clone());
                if let Ok(Token::Unknown) = res {
                    break;
                }

                let ret = match res {
                    Ok(t) => self.eval_token(t),
                    Err(err) => Err(err),
                };
                return ret;
            }
        }

        Ok(token)
    }

    fn match_token(&self, signature: Token, value: Token) -> bool {
        match (signature, value) {
            (Token::Lst(a), Token::Lst(b)) => {
                if a.len() != b.len() {
                    false
                } else {
                    let mut ret = true;
                    for i in 0..a.len() {
                        ret = ret && self.match_token(a[i].clone(), b[i].clone());
                    }

                    ret
                }
            }
            (Token::Var(_), _) => true,
            (Token::Str(a), Token::Str(b)) => a == b,
            (Token::Keyword(a), Token::Keyword(b)) => a == b,
            (Token::Number(a), Token::Number(b)) => a == b,
            (Token::Bool(a), Token::Bool(b)) => a == b,
            (Token::BinaryOp(a), Token::BinaryOp(b)) => a == b,
            (Token::UnaryOp(a), Token::UnaryOp(b)) => a == b,
            (Token::Wildcard(_), _) => true,
            _ => false,
        }
    }

    pub fn eval_token(&mut self, token: Token) -> EvalResult {
        let mut child_eval = Evaluator::new();

        // child_eval.curr_state = self.curr_state.clone();

        let ret = match token {
            Token::Lst(lst) => self.eval_lst(lst),
            t => Ok(t),
        };

        // println!("current state: ");
        // for item in self.curr_state.definition.clone() {
        //     println!("{:?}", item.signature);
        // }
        // println!("end state ");

        ret
    }
}
