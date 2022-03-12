#[derive(Debug, Clone)]
pub enum Token {
    LST(Vec<Token>), // list
    VAR(String),     // variable
    STR(String),     // string
    INT(i32),        // integer
    FLOAT(f32),      // integer
    BOOL(bool),      // booelan
    SYMBOL(String),  // symbol like operator
    UNKNOWN,
}
