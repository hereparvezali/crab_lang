use std::{iter::Peekable, str::Chars};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Let,
    While,
    If,
    Elif,
    Else,
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
    LBrace,
    RBrace,
    Semicolon,
    EqualEqual,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
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
                        "while" => Token::While,
                        "if" => Token::If,
                        "elif" => Token::Elif,
                        "else" => Token::Else,
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
                    let mut curr_token = Token::Equal;
                    self.input.next();
                    if let Some(&c) = self.input.peek() {
                        if c == '=' {
                            curr_token = Token::EqualEqual;
                            self.input.next();
                        }
                    }
                    tokens.push(curr_token);
                }
                '>' => {
                    let mut curr_token = Token::Greater;
                    self.input.next();
                    if let Some(&c) = self.input.peek() {
                        if c == '=' {
                            curr_token = Token::GreaterEqual;
                        }
                    }
                    tokens.push(curr_token);
                }
                '<' => {
                    let mut curr_token = Token::Less;
                    self.input.next();
                    if let Some(&c) = self.input.peek() {
                        if c == '=' {
                            curr_token = Token::LessEqual;
                        }
                    }
                    tokens.push(curr_token);
                }
                '!' => {
                    self.input.next();
                    if self.input.peek() != Some(&'=') {
                        panic!("Unexpected token");
                    }
                    self.input.next();
                    tokens.push(Token::NotEqual);
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
                '{' => {
                    tokens.push(Token::LBrace);
                    self.input.next();
                }
                '}' => {
                    tokens.push(Token::RBrace);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let source = "let x = 5; let y = x+(4+2)/2; let z = x+y; exit z + 2;";
        let tokens = Lexer::new(source).tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::Let,
                Token::Ident("x".to_string()),
                Token::Equal,
                Token::Number(5),
                Token::Semicolon,
                Token::Let,
                Token::Ident("y".to_string()),
                Token::Equal,
                Token::Ident("x".to_string()),
                Token::Plus,
                Token::LParen,
                Token::Number(4),
                Token::Plus,
                Token::Number(2),
                Token::RParen,
                Token::Slash,
                Token::Number(2),
                Token::Semicolon,
                Token::Let,
                Token::Ident("z".to_string()),
                Token::Equal,
                Token::Ident("x".to_string()),
                Token::Plus,
                Token::Ident("y".to_string()),
                Token::Semicolon,
                Token::Exit,
                Token::Ident("z".to_string()),
                Token::Plus,
                Token::Number(2),
                Token::Semicolon,
            ]
        );
    }
}
