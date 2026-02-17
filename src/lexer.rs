use std::{iter::Peekable, str::Chars};

#[derive(Debug, Clone)]
pub enum Token {
    Let,
    Exit,
    Ident(String),
    Number(i32),
    Equal,
    Plus,
    Minus,
    Asterisk,
    Slash,
    LParen,
    RParen,
    Semicolon,
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}
impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
        }
    }
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        while let Some(&c) = self.input.peek() {
            match c {
                'a'..='z' | 'A'..='Z' => {
                    let mut identifier = String::new();
                    while let Some(&c @ ('a'..='z' | 'A'..='Z' | '0'..='9' | '_')) =
                        self.input.peek()
                    {
                        identifier.push(c);
                        self.input.next();
                    }
                    tokens.push(match identifier.as_str() {
                        "let" => Token::Let,
                        "exit" => Token::Exit,
                        _ => Token::Ident(identifier),
                    });
                }
                '0'..='9' => {
                    let mut number = 0;
                    while let Some(&c @ ('0'..='9')) = self.input.peek() {
                        number = number * 10 + (c as i32 - '0' as i32);
                        self.input.next();
                    }
                    tokens.push(Token::Number(number));
                }
                '=' => {
                    tokens.push(Token::Equal);
                    self.input.next();
                }
                '+' => {
                    tokens.push(Token::Plus);
                    self.input.next();
                }
                '-' => {
                    tokens.push(Token::Minus);
                    self.input.next();
                }
                '*' => {
                    tokens.push(Token::Asterisk);
                    self.input.next();
                }
                '/' => {
                    tokens.push(Token::Slash);
                    self.input.next();
                }
                '(' => {
                    tokens.push(Token::LParen);
                    self.input.next();
                }
                ')' => {
                    tokens.push(Token::RParen);
                    self.input.next();
                }
                ';' => {
                    tokens.push(Token::Semicolon);
                    self.input.next();
                }
                ' ' | '\n' | '\t' | '\r' => {
                    self.input.next();
                }
                _ => {
                    self.input.next();
                }
            }
        }
        tokens
    }
}
