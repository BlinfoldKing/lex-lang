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

    let mut states = input.clone();
    if let Some(State::Symbol(sym, _, _)) = states.first() {
        if *sym == '-' {
            num.push(*sym);
            states.remove(0);
        }
    }

    for state in states {
        match state {
            State::Num(c, _, _) => num.push(c),
            State::Symbol(c, lin, col) => match c {
                '.' => {
                    println!("masuk");
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
        Ok(Token::FLOAT(num.parse().unwrap()))
    } else {
        Ok(Token::INT(num.parse().unwrap()))
    }
}

fn parse_unquoted_string(states: Vec<State>) -> Result<Token, ParseError> {
    match states.first() {
        Some(State::Char(c, _, _)) => {
            match c {
                'A'..='Z' => {
                    return parse_variable(states);
                }
                _ => {}
            };
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
        Ok(Token::BOOL(true))
    } else if st == "false" {
        Ok(Token::BOOL(true))
    } else {
        Ok(Token::STR(st))
    }
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

    return Ok(Token::VAR(st));
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
        "+" | "-" | "<" | ">" | "<=" | ">=" | "!" | "*" | "=" => Ok(Token::SYMBOL(sym)),
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
                        if *c == '-' {
                            parse_number(accumulator)
                        } else {
                            parse_symbol(accumulator)
                        }
                    }
                    _ => Ok(Token::UNKNOWN),
                };
                match parse_result {
                    Ok(Token::UNKNOWN) => {}
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

    Ok(Token::LST(lst))
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
                        substate.insert(0, states.pop().unwrap());
                        match states.last() {
                            Some(State::LstStart(_, _)) => {
                                break;
                            }
                            None => {
                                return Err(ParseError::MissingBracket(line));
                            }
                            _ => {}
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
                '+' | '-' | '=' | '>' | '<' | '.' | '_' => {
                    states.push(State::Symbol(c, line, collumn));
                }
                _ => return Err(ParseError::InvalidCharacter(c, line, collumn)),
            };
        }

        let mut result: Vec<Token> = vec![];

        for state in states {
            match state {
                State::Parsed(token) => result.push(token),
                _ => return Err(ParseError::Other(line)),
            }
        }

        Ok(Token::LST(result))
    }
}
