use crate::lib::token::Token;

pub fn print_token(token: Token) {
    print_token_with_depth(token, 0);
    println!();
}

pub fn print_token_with_depth(token: Token, depth: i32) {
    match token {
        Token::Lst(lst) => print_lst(lst, depth),
        Token::Str(str) => print!("{} ", str),
        Token::Var(var) => print!("{} ", var),
        Token::Number(num) => print!("{} ", num),
        Token::Bool(b) => print!("{} ", b),
        Token::Keyword(keyword) => print!("{} ", keyword),
        Token::Wildcard(wildcard) => print!("{} ", wildcard),
        Token::BinaryOp(op) => print!("{} ", op),
        Token::UnaryOp(op) => print!("{} ", op),
        _ => print!(" "),
    }
}

fn print_lst(token: Vec<Token>, depth: i32) {
    if depth > 0 {
        print!("\n");
    }
    for _ in 0..depth {
        print!("  ");
    }
    print!("( ");
    for item in token {
        print_token_with_depth(item, depth + 1);
    }
    print!(") ");
}
