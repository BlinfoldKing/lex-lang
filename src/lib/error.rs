use std::fmt;

#[derive(Debug, Clone)]
pub enum LexError {
    ParseError(ParseError),
    EvalError(EvalError),
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexError::ParseError(err) => err.fmt(f),
            LexError::EvalError(err) => err.fmt(f),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ParseError {
    InvalidCharacter(char, i32, i32),
    InvalidSymbol(String, i32, i32),
    MissingBracket(i32),
    Other(i32),
    UnknownError,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::InvalidCharacter(c, line, collumn) => write!(
                f,
                "invalid character \"{}\" at line {}:{}",
                c, line, collumn
            ),
            _ => write!(f, "unknown parse error"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum EvalError {
    UnknownOperator,
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            _ => write!(f, "unknown error"),
        }
    }
}
