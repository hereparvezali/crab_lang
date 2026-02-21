pub mod codegen;
pub mod lexer;
pub mod parser;

use std::fs::{read_to_string, write};

use crate::{codegen::CodeGen, lexer::Lexer, parser::Parser};

fn main() {
    let source = read_to_string("./test.txt").unwrap();
    let tokens = Lexer::new(&source).tokenize();
    println!("{:?}", tokens);
    let stmts = Parser::new(tokens).parse();
    println!("{:?}", stmts);
    let asm = CodeGen::new().generate(&stmts);
    write("./output.asm", &asm).expect("failed to write output.asm");
    println!("Done");
}
