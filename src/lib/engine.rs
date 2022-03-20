use crate::lib::error::LexError;
use crate::lib::evaluator::{
    definition::{def::Def, math::Math, misc::Misc},
    Evaluator,
};
use crate::lib::parser::Parser;
use crate::lib::token::Token;

pub struct Engine {
    parser: Parser,
    evalutator: Evaluator,
}

impl Engine {
    pub fn new() -> Self {
        let mut e = Engine {
            parser: Parser {},
            evalutator: Evaluator::new(),
        };

        e.evalutator.load(Misc {});
        e.evalutator.load(Def {});
        e.evalutator.load(Math {});
        e
    }

    pub fn parse(&mut self, input: String) -> Result<Token, LexError> {
        let mut str = input.clone();
        str.push(')');
        str.insert(0, '(');
        match self.parser.parse(str) {
            Ok(ast) => match self.evalutator.eval_token(ast) {
                Ok(token) => Ok(token),
                Err(err) => Err(LexError::EvalError(err)),
            },
            Err(e) => Err(LexError::ParseError(e)),
        }
    }
}
