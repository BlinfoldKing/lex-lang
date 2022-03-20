use crate::lib::error::ParseError;
use crate::lib::token::Token;

fn is_newline(c: char) -> bool {
    c == '\n'
}

#[derive(Debug, Clone)]
enum State {
    Char(char, i32, i32),
    Num(char, i32, i32),
    Symbol(char, i32, i32),
    LstStart(i32, i32),
    Whitespace(i32, i32),
    Parsed(Token),
}

fn parse_number(input: Vec<State>) -> Result<Token, ParseError> {
    let mut is_float: bool = false;
    let mut num = String::new();

    let states = input.clone();
    // TODO: handle negatives
    //     if let Some(State::Symbol(sym, _, _)) = states.first() {
    //         if *sym == '-' {
    //             num.push(*sym);
    //             states.remove(0);
    //         }
    //     }

    for state in states {
        match state {
            State::Num(c, _, _) => num.push(c),
            State::Symbol(c, lin, col) => match c {
                '.' => {
                    if !is_float {
                        is_float = true;
                        num.push(c);
                    } else {
                        return Err(ParseError::InvalidCharacter(c, lin, col));
                    }
                }
                _ => return Err(ParseError::InvalidCharacter(c, lin, col)),
            },
            State::Char(c, lin, col) => return Err(ParseError::InvalidCharacter(c, lin, col)),
            _ => {}
        };
    }

    if is_float {
        Ok(Token::Number(num.parse().unwrap()))
    } else {
        Ok(Token::Number(num.parse().unwrap()))
    }
}

fn parse_quoted_string(input: Vec<State>) -> Result<Token, ParseError> {
    let mut states = input.clone();
    match states.first() {
        Some(State::Symbol('"', _, _)) => {
            if states.len() > 1 {
                if let Some(State::Symbol('"', _, _)) = states.last() {
                    states.remove(0);
                    states.pop();
                }
            }
        }
        _ => {}
    };
    let mut st = String::new();
    for state in states {
        match state {
            State::Char(c, _, _) => st.push(c),
            State::Symbol(c, _, _) => st.push(c),
            State::Num(c, _, _) => st.push(c),
            _ => return Err(ParseError::UnknownError),
        };
    }

    Ok(Token::Str(st))
}
fn parse_unquoted_string(input: Vec<State>) -> Result<Token, ParseError> {
    let states = input.clone();
    match states.first() {
        Some(State::Char(c, _, _)) => {
            match c {
                'A'..='Z' => {
                    return parse_variable(states);
                }
                _ => {}
            };
        }
        Some(State::Symbol('.', _, _)) => {
            return parse_keyword(states);
        }
        Some(State::Symbol('_', _, _)) => {
            return parse_wildcard(states);
        }
        _ => {}
    };

    let mut st = String::new();
    for state in states {
        match state {
            State::Char(c, _, _) => st.push(c),
            State::Symbol(c, lin, col) => {
                match c {
                    '-' | '_' => st.push(c),
                    _ => return Err(ParseError::InvalidCharacter(c, lin, col)),
                };
            }
            State::Num(c, _, _) => st.push(c),
            _ => return Err(ParseError::UnknownError),
        };
    }

    if st == "true" {
        Ok(Token::Bool(true))
    } else if st == "false" {
        Ok(Token::Bool(false))
    } else {
        Ok(Token::Str(st))
    }
}

fn parse_wildcard(input: Vec<State>) -> Result<Token, ParseError> {
    let mut st = String::new();

    let mut states = input.clone();
    if let Some(State::Symbol('_', _, _)) = states.first() {
        st.push('_');
        states.remove(0);
    }

    for state in states {
        match state {
            State::Char(c, _, _) => st.push(c),
            State::Symbol(c, lin, col) => {
                match c {
                    '-' | '_' => st.push(c),
                    _ => return Err(ParseError::InvalidCharacter(c, lin, col)),
                };
            }
            State::Num(c, _, _) => st.push(c),
            _ => return Err(ParseError::UnknownError),
        };
    }

    return Ok(Token::Wildcard(st));
}

fn parse_keyword(input: Vec<State>) -> Result<Token, ParseError> {
    let mut st = String::new();

    let mut states = input.clone();
    if let Some(State::Symbol('.', _, _)) = states.first() {
        st.push('.');
        states.remove(0);
    }

    for state in states {
        match state {
            State::Char(c, _, _) => st.push(c),
            State::Symbol(c, lin, col) => {
                match c {
                    '-' | '_' => st.push(c),
                    _ => return Err(ParseError::InvalidCharacter(c, lin, col)),
                };
            }
            State::Num(c, _, _) => st.push(c),
            _ => return Err(ParseError::UnknownError),
        };
    }

    return Ok(Token::Keyword(st));
}

fn parse_variable(states: Vec<State>) -> Result<Token, ParseError> {
    let mut st = String::new();
    for state in states {
        match state {
            State::Char(c, _, _) => st.push(c),
            State::Symbol(c, lin, col) => {
                match c {
                    '-' | '_' => st.push(c),
                    _ => return Err(ParseError::InvalidCharacter(c, lin, col)),
                };
            }
            State::Num(c, _, _) => st.push(c),
            _ => return Err(ParseError::UnknownError),
        };
    }

    return Ok(Token::Var(st));
}

fn parse_symbol(input: Vec<State>) -> Result<Token, ParseError> {
    let mut sym = String::new();
    let states = input.clone();
    for state in states {
        match state {
            State::Symbol(c, _, _) => sym.push(c),
            State::Char(c, _, _) => sym.push(c),
            State::Num(c, _, _) => sym.push(c),
            _ => return Err(ParseError::UnknownError),
        };
    }

    match &*sym {
        "+" | "-" | "<" | ">" | "<=" | ">=" | "*" | "=" | "**" | "%" | "/" => {
            Ok(Token::BinaryOp(sym))
        }
        "!" => Ok(Token::UnaryOp(sym)),
        _ => Err(ParseError::UnknownError),
    }
}

fn parse_list(input: Vec<State>) -> Result<Token, ParseError> {
    let mut states = input.clone();
    states.push(State::Whitespace(0, 0));

    let mut lst: Vec<Token> = vec![];
    let mut accumulator: Vec<State> = vec![];
    for state in states {
        match state {
            State::Whitespace(_, _) => {
                let parse_result = match accumulator.first() {
                    Some(State::Char(_, _, _)) => parse_unquoted_string(accumulator),
                    Some(State::Num(_, _, _)) => parse_number(accumulator),
                    Some(State::Symbol(c, _, _)) => {
                        if *c == '.' || *c == '_' {
                            parse_unquoted_string(accumulator)
                        } else if *c == '"' {
                            parse_quoted_string(accumulator)
                        } else {
                            parse_symbol(accumulator)
                        }
                    }
                    _ => Ok(Token::Unknown),
                };
                match parse_result {
                    Ok(Token::Unknown) => {}
                    Ok(token) => lst.push(token),
                    Err(err) => return Err(err),
                };
                accumulator = vec![];
            }
            State::Char(_, _, _) => accumulator.push(state),
            State::Symbol(_, _, _) => accumulator.push(state),
            State::Num(_, _, _) => accumulator.push(state),
            State::Parsed(token) => {
                lst.push(token);
            }
            _ => {}
        }
    }

    Ok(Token::Lst(lst))
}

pub struct Parser {}

impl Parser {
    pub fn parse(&self, input: String) -> Result<Token, ParseError> {
        let mut line: i32 = 1;
        let mut collumn: i32 = 0;

        let mut states: Vec<State> = vec![];

        for c in input.chars() {
            if is_newline(c) {
                line += 1;
                continue;
            }

            collumn += 1;

            match c {
                '(' => {
                    states.push(State::LstStart(line, collumn));
                }
                ')' => {
                    let mut substate: Vec<State> = vec![];
                    loop {
                        match states.last() {
                            Some(State::LstStart(_, _)) => {
                                break;
                            }
                            None => {
                                return Err(ParseError::MissingBracket(line));
                            }
                            _ => {
                                substate.insert(0, states.pop().unwrap());
                            }
                        }
                    }

                    states.pop();

                    match parse_list(substate) {
                        Ok(token) => states.push(State::Parsed(token)),
                        Err(err) => return Err(err),
                    };
                }
                ' ' => match states.last() {
                    Some(State::Whitespace(_, _)) => {}
                    _ => {
                        states.push(State::Whitespace(line, collumn));
                    }
                },
                'A'..='Z' | 'a'..='z' => {
                    states.push(State::Char(c, line, collumn));
                }
                '0'..='9' => {
                    states.push(State::Num(c, line, collumn));
                }
                _ => {
                    states.push(State::Symbol(c, line, collumn));
                }
            };
        }

        let mut result: Vec<Token> = vec![];

        for state in states {
            match state {
                State::Parsed(token) => result.push(token),
                _ => return Err(ParseError::UnknownError),
            }
        }

        Ok(result.first().unwrap().clone())
    }
}
