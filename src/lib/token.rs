#[derive(Debug, Clone)]
pub enum Token {
    Lst(Vec<Token>),  // list
    Var(String),      // variable
    Str(String),      // string
    Keyword(String),  // string
    Number(f32),      // integer
    Bool(bool),       // booelan
    BinaryOp(String), // symbol like operator
    UnaryOp(String),  // symbol like operator
    Wildcard(String),
    Unknown,
}
