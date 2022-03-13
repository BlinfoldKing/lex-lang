use crate::lib::error::EvalError;
use crate::lib::parser::Parser;
use crate::lib::printer::print_token;
use crate::lib::token::Operator;
use crate::lib::token::Token;

use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone)]
pub struct EvalState {
    declaration: Vec<Token>,
    definition: Vec<(Token, Token)>,
    variables: HashMap<String, Token>,
    return_value: Option<Token>,
}

impl EvalState {
    pub fn new() -> Self {
        EvalState {
            declaration: vec![],
            definition: vec![],
            variables: HashMap::new(),
            return_value: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Evaluator {
    parent_state: Option<EvalState>,
    curr_state: EvalState,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            parent_state: None,
            curr_state: EvalState::new(),
        }
    }

    fn eval_lst(&mut self, input: Vec<Token>) -> Result<Token, EvalError> {
        let lst: Vec<Token> = input
            .into_iter()
            .map(|item| match item {
                Token::Lst(l) => self.eval_lst(l).unwrap(),
                _ => item,
            })
            .collect();

        let res = match &lst[..] {
            [Token::Symbol(Operator::BinaryOperator(op)), x, y] => {
                self.eval_binary_operator(x, op.to_owned(), y)
            }
            [Token::Symbol(Operator::UnaryOperator(op)), x] => {
                self.eval_unary_operator(x, op.to_owned())
            }
            [Token::Keyword(keyword), x] => self.eval_unary_keyword(keyword.to_owned(), x),
            [Token::Keyword(keyword), x, y] => self.eval_binary_keyword(keyword.to_owned(), x, y),
            _ => Ok(Token::Unknown),
        };

        match &self.parent_state {
            None => (),
            Some(parent) => match &parent.return_value {
                None => (),
                Some(ret) => return Ok(ret.clone()),
            },
        };

        match res {
            Ok(Token::Unknown) => Ok(Token::Lst(lst.clone())),
            r => r,
        }
    }

    fn eval_binary_keyword(
        &mut self,
        keyword: String,
        a: &Token,
        b: &Token,
    ) -> Result<Token, EvalError> {
        match (&*keyword, (a, b)) {
            // (".is", (Token::Var(variable), value)) => {
            // },
            (".and", (Token::Bool(a), Token::Bool(b))) => Ok(Token::Bool(*a && *b)),
            // (".and", (a, b)) => {
            //     print_token(a.clone());
            //     print_token(b.clone());
            //     Ok(Token::Bool(true))
            // },
            (".or", (Token::Bool(a), Token::Bool(b))) => Ok(Token::Bool(*a || *b)),
            (".union", (Token::Lst(a), Token::Lst(b))) => {
                // let mut ret: Vec<Token> = vec![Token::Lst(a.to_vec()),Token::Lst(b.to_vec())];
                let mut ret: Vec<Token> = vec![];
                for item in a {
                    ret.push(item.clone());
                }

                for item in b {
                    ret.push(item.clone());
                }
                Ok(Token::Lst(ret))
            }
            _ => Ok(Token::Unknown),
        }
    }

    fn eval_unary_keyword(&mut self, keyword: String, value: &Token) -> Result<Token, EvalError> {
        match (&*keyword, value) {
            (".print", x) => {
                print_token(x.clone());
                Ok(Token::Bool(true))
            }
            (".return", x) => {
                match &self.parent_state {
                    Some(parent) => {
                        let mut newstate = parent.clone();
                        newstate.return_value = Some(x.clone());

                        self.parent_state = Some(newstate);
                    }
                    None => (),
                }
                Ok(x.clone())
            }
            (".load", Token::Str(filepath)) => match fs::read_to_string(filepath) {
                Ok(file) => {
                    let p = Parser {};
                    match p.parse(file) {
                        Err(_) => Err(EvalError::UnknownOperator),
                        Ok(token) => self.eval_token(token),
                    }
                }
                Err(_) => Ok(Token::Bool(false)),
            },
            (".declare", x) | (".dec", x) => {
                match &self.parent_state {
                    Some(parent) => {
                        let mut newstate = parent.clone();
                        newstate.declaration.push(x.clone());

                        self.parent_state = Some(newstate);
                    }
                    None => (),
                };
                Ok(Token::Bool(true))
            }
            (".match", x) => {
                let mut res: Vec<Token> = vec![];
                let state = &self.parent_state.clone().unwrap();
                for rule in state.declaration.clone() {
                    match self.match_declaration(x, &rule) {
                        Ok(Token::Bool(false)) => continue,
                        Ok(token) => res.push(token),
                        Err(err) => return Err(err),
                    }
                }
                Ok(Token::Lst(res))
            }
            (".eval", x) => self.eval_query(x.clone()),
            (".head", Token::Lst(tokens)) => Ok(tokens.first().unwrap().clone()),
            (".back", Token::Lst(tokens)) => Ok(tokens.last().unwrap().clone()),
            _ => Err(EvalError::UnknownOperator),
        }
    }

    fn eval_query(&mut self, token: Token) -> Result<Token, EvalError> {
        let mut declarations: Vec<Token> = vec![];
        if let Some(state) = &mut self.parent_state {
            declarations.append(&mut state.declaration.clone())
        }
        declarations.append(&mut self.curr_state.declaration.clone());

        match token {
            Token::Lst(lst) => {
                let mut new_lst: Vec<Token> = vec![];
                for elem in lst {
                    match elem {
                        Token::Lst(_) => {
                            let sublst = self.eval_query(elem);
                            match sublst {
                                Ok(sub) => new_lst.push(sub),
                                err => return err,
                            }
                        }
                        e => new_lst.push(e),
                    }
                }

                let mut valid_facts: Vec<Token> = vec![];
                for fact in declarations {
                    let mut valid = true;
                    match fact.clone() {
                        Token::Lst(fact_lst) => {
                            if fact_lst.len() != new_lst.len() {
                                continue;
                            }

                            for i in 0..fact_lst.len() {
                                match (new_lst[i].clone(), fact_lst[i].clone()) {
                                    (Token::Var(_), _) => continue,
                                    (Token::Wildcard(_), _) => continue,
                                    (x, y) => {
                                        let compare = vec![
                                            Token::Symbol(Operator::BinaryOperator("=".to_owned())),
                                            x.clone(),
                                            y,
                                        ];
                                        let comparisson = self.eval_lst(compare.clone());

                                        if let Ok(Token::Bool(matched)) = comparisson {
                                            if !matched {
                                                valid = false;
                                                break;
                                                // return Token::Bool(false);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        _ => continue,
                    }

                    if valid {
                        valid_facts.push(fact.clone());
                    }
                }


                let mut ret: Vec<Token> = vec![];
                for fact in &valid_facts {
                    let mut vars: Vec<Token> = vec![];
                    if let Token::Lst(lst) = fact {
                        for (i, item) in new_lst.clone().into_iter().enumerate() {
                            if let Token::Var(_) = item.clone() {
                                vars.push(Token::Lst(vec![
                                    Token::Symbol(Operator::BinaryOperator("=".to_owned())),
                                    item,
                                    lst[i].clone(),
                                ]));
                            }
                        }
                    }
                    ret.push(Token::Lst(vars));
                }

                if valid_facts.len() == 0 {
                    let token = match &new_lst[..] {
                        [Token::Keyword(keyword), a, b] => {
                            let kw: &str = keyword;
                            match kw {
                                ".or" => Token::Lst(vec![Token::Keyword(".union".to_owned()), a.clone(), b.clone()]),
                                _ => Token::Lst(new_lst.clone())
                            }
                        },
                        _ => Token::Lst(new_lst.clone())
                    };
                    self.eval_token(token)
                } else {
                    Ok(Token::Lst(ret))
                }
            }
            t => Ok(t),
        }
    }

    fn compare_list(&mut self, a: Vec<Token>, b: Vec<Token>) -> Result<Token, EvalError> {
        let mut ret: Vec<Token> = vec![];

        for i in 0..a.len() {
            match (a[i].clone(), b[i].clone()) {
                (Token::Var(_), value) => ret.push(value),
                (value, Token::Var(_)) => ret.push(value),
                (x, y) => {
                    let compare = vec![
                        Token::Symbol(Operator::BinaryOperator("=".to_owned())),
                        x.clone(),
                        y,
                    ];
                    let comparisson = self.eval_lst(compare.clone());
                    if let Ok(Token::Bool(matched)) = comparisson {
                        if matched {
                            ret.push(x)
                        } else {
                            return Ok(Token::Bool(false));
                        }
                    }
                }
            };
        }

        Ok(Token::Lst(ret))
    }

    fn match_declaration(&mut self, source: &Token, target: &Token) -> Result<Token, EvalError> {
        match (source, target) {
            (Token::Lst(s), Token::Lst(t)) => {
                if s.len() != t.len() {
                    Ok(Token::Bool(false))
                } else {
                    self.compare_list(s.to_vec(), t.to_vec())
                }
            }
            _ => Ok(Token::Bool(false)),
        }
    }

    fn eval_unary_operator(&self, x: &Token, op: String) -> Result<Token, EvalError> {
        let res = match x {
            Token::Bool(a) => match &*op {
                "!" => Token::Bool(!a),
                _ => Token::Unknown,
            },
            _ => Token::Unknown,
        };

        match res {
            Token::Unknown => Err(EvalError::UnknownOperator),
            token => Ok(token),
        }
    }

    fn eval_binary_operator(&self, x: &Token, op: String, y: &Token) -> Result<Token, EvalError> {
        let res = match (x, y) {
            (Token::Number(a), Token::Number(b)) => match &*op {
                "+" => Token::Number(a + b),
                "-" => Token::Number(a + b),
                "/" => Token::Number(a + b),
                "*" => Token::Number(a + b),
                "=" => Token::Bool(a == b),
                ">=" => Token::Bool(a >= b),
                "<=" => Token::Bool(a <= b),
                _ => Token::Unknown,
            },
            (Token::Str(a), Token::Str(b)) => match &*op {
                "=" => Token::Bool(a == b),
                _ => Token::Unknown,
            },
            (Token::Bool(a), Token::Bool(b)) => match &*op {
                "=" => Token::Bool(a == b),
                _ => Token::Unknown,
            },
            (
                Token::Symbol(Operator::BinaryOperator(a)),
                Token::Symbol(Operator::BinaryOperator(b)),
            ) => match &*op {
                "=" => Token::Bool(a == b),
                _ => Token::Unknown,
            },
            (
                Token::Symbol(Operator::UnaryOperator(a)),
                Token::Symbol(Operator::UnaryOperator(b)),
            ) => match &*op {
                "=" => Token::Bool(a == b),
                _ => Token::Unknown,
            },
            (Token::Var(a), Token::Var(b)) => match &*op {
                "=" => Token::Bool(a == b),
                _ => Token::Unknown,
            },
            (Token::Var(_), _) => match &*op {
                _ => Token::Unknown,
            },
            (_, _) => match &*op {
                "=" => Token::Bool(false),
                _ => Token::Unknown
            }
        };

        Ok(res)
    }

    fn merge_state(&mut self, newstate: EvalState) -> Result<(), EvalError> {
        self.curr_state.declaration = vec![];
        for declaration in newstate.declaration {
            self.curr_state.declaration.push(declaration);
        }

        let mut new_variables: HashMap<String, Token> = HashMap::new();

        for (key, value) in newstate.variables {
            new_variables.insert(key, value);
        }

        for (key, value) in self.curr_state.variables.clone() {
            match new_variables.get(&key) {
                None => {
                    new_variables.insert(key, value);
                }
                _ => (),
            }
        }

        self.curr_state.variables = new_variables;

        Ok(())
    }

    pub fn eval_token(&mut self, token: Token) -> Result<Token, EvalError> {
        let mut child_eval = Evaluator::new();

        child_eval.parent_state = Some(self.curr_state.clone());

        let ret = match token {
            Token::Lst(lst) => child_eval.eval_lst(lst),
            t => Ok(t),
        };

        if let Err(err) = self.merge_state(child_eval.parent_state.unwrap()) {
            return Err(err);
        }

        ret
    }
}
