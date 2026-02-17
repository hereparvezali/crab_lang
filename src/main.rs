pub mod lexer;
pub mod parser;

use std::fs::read_to_string;

use crate::{lexer::Lexer, parser::Parser};

fn main() {
    let source = read_to_string("./test.txt").unwrap();
    let stmts = Parser::new(Lexer::new(&source).tokenize()).parse();
    println!("{:?}", stmts);
}
