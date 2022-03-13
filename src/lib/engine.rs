use crate::lib::error::LexError;
use crate::lib::evaluator::Evaluator;
use crate::lib::parser::Parser;
use crate::lib::token::Token;

pub struct Engine {
    parser: Parser,
    evalutator: Evaluator,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            parser: Parser {},
            evalutator: Evaluator::new(),
        }
    }

    pub fn parse(&mut self, input: String) -> Result<Token, LexError> {
        let mut str = input.clone();
        str.push(')');
        str.insert(0, '(');
        match self.parser.parse(str) {
            Ok(ast) => {
                // println!("{:?}", ast);

                match self.evalutator.eval_token(ast) {
                    Ok(token) => Ok(token),
                    Err(err) => Err(LexError::EvalError(err)),
                }
            }
            Err(e) => Err(LexError::ParseError(e)),
        }
    }
}
