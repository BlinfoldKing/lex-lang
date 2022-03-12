use crate::lib::error::LexError;
use crate::lib::parser::Parser;

pub struct Engine {
    parser: Parser,
}

impl Engine {
    pub fn new() -> Self {
        Engine { parser: Parser {} }
    }

    pub fn parse(&self, input: String) -> Result<(), LexError> {
        match self.parser.parse(input) {
            Ok(ast) => {
                println!("{:?}", ast);
                Ok(())
            }
            Err(e) => Err(LexError::ParseError(e)),
        }
    }
}
