#[derive(Debug, Clone)]
pub enum Token {
    Lst(Vec<Token>),  // list
    Var(String),      // variable
    Str(String),      // string
    Keyword(String),  // string
    Number(f32),      // integer
    Bool(bool),       // booelan
    Symbol(Operator), // symbol like operator
    Wildcard(String),
    Unknown,
}

#[derive(Debug, Clone)]
pub enum Operator {
    BinaryOperator(String),
    UnaryOperator(String),
}
