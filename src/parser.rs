use std::iter::Peekable;

use crate::lexer::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Ident(String),
    Num(i32),
    BinOp(Box<Expr>, Op, Box<Expr>),
    UnaryOp(Op, Box<Expr>),
}
#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    NotEq,
    Gt,
    Gte,
    Lt,
    Lte,
}
#[derive(Debug, Clone)]
pub enum Stmt {
    Let(String, Expr),
    Exit(Expr),
    While(Expr, Vec<Stmt>),
    If(Expr, Vec<Stmt>, Vec<(Expr, Vec<Stmt>)>, Option<Vec<Stmt>>),
}

pub struct Parser {
    tokens: Peekable<std::vec::IntoIter<Token>>,
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
        }
    }
    fn expect(&mut self, expected: Token) {
        match self.tokens.next() {
            Some(t) => {
                if std::mem::discriminant(&t) != std::mem::discriminant(&expected) {
                    panic!("expected {:?}, found {:?}", expected, t);
                }
            }
            None => {
                panic!("expected {:?}, found EOF", expected);
            }
        }
    }
    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while let Some(t) = self.tokens.peek().cloned() {
            match t {
                Token::Let => {
                    self.tokens.next();
                    let ident = match self.tokens.next() {
                        Some(Token::Ident(id)) => id,
                        other => {
                            panic!("expected Identifier, got {:?}", other);
                        }
                    };
                    self.expect(Token::Equal);
                    let expr = self.parse_expr();
                    self.expect(Token::Semicolon);
                    stmts.push(Stmt::Let(ident, expr));
                }
                Token::Exit => {
                    self.tokens.next();
                    self.expect(Token::LParen);
                    let expr = self.parse_expr();
                    self.expect(Token::RParen);
                    self.expect(Token::Semicolon);
                    stmts.push(Stmt::Exit(expr));
                }
                Token::While => {
                    self.tokens.next();
                    self.expect(Token::LParen);
                    let cond = self.parse_expr();
                    self.expect(Token::RParen);
                    self.expect(Token::LBrace);
                    let block_stmts = self.parse();
                    self.expect(Token::RBrace);
                    stmts.push(Stmt::While(cond, block_stmts));
                }
                Token::If => {
                    self.tokens.next();
                    self.expect(Token::LParen);
                    let cond = self.parse_expr();
                    self.expect(Token::RParen);
                    self.expect(Token::LBrace);
                    let block_stmts = self.parse();
                    self.expect(Token::RBrace);

                    let mut elifs = Vec::new();
                    while let Some(_elif @ Token::Elif) = self.tokens.peek() {
                        self.tokens.next();
                        self.expect(Token::LParen);
                        let elif_cond = self.parse_expr();
                        self.expect(Token::RParen);

                        self.expect(Token::LBrace);
                        let elif_block_stmts = self.parse();
                        self.expect(Token::RBrace);
                        elifs.push((elif_cond, elif_block_stmts));
                    }
                    let mut else_block_stmts = None;
                    if let Some(_els @ Token::Else) = self.tokens.peek() {
                        self.tokens.next();
                        self.expect(Token::LBrace);
                        else_block_stmts = Some(self.parse());
                        self.expect(Token::RBrace);
                    }
                    stmts.push(Stmt::If(cond, block_stmts, elifs, else_block_stmts));
                }
                Token::RBrace => {
                    return stmts;
                }
                tok => {
                    panic!("unexpected token {:?}", tok);
                }
            }
            println!("{:?}", stmts);
        }
        stmts
    }
    fn parse_expr(&mut self) -> Expr {
        self.parse_comparison()
    }
    fn parse_comparison(&mut self) -> Expr {
        let mut left = self.parse_add();
        while let Some(
            t @ (Token::EqualEqual
            | Token::NotEqual
            | Token::Greater
            | Token::GreaterEqual
            | Token::Less
            | Token::LessEqual),
        ) = self.tokens.peek().cloned()
        {
            let op = match t {
                Token::EqualEqual => Op::Eq,
                Token::NotEqual => Op::NotEq,
                Token::Greater => Op::Gt,
                Token::GreaterEqual => Op::Gte,
                Token::Less => Op::Lt,
                Token::LessEqual => Op::Lte,
                _ => unreachable!(),
            };
            self.tokens.next();
            let right = self.parse_add();
            left = Expr::BinOp(Box::new(left), op, Box::new(right));
        }
        left
    }
    fn parse_add(&mut self) -> Expr {
        let mut left = self.parse_mul();
        while let Some(t @ (Token::Plus | Token::Minus)) = self.tokens.peek().cloned() {
            let op = match t {
                Token::Plus => Op::Add,
                Token::Minus => Op::Sub,
                _ => unreachable!(),
            };
            self.tokens.next();
            let right = self.parse_mul();
            left = Expr::BinOp(Box::new(left), op, Box::new(right));
        }
        left
    }
    fn parse_mul(&mut self) -> Expr {
        let mut left = self.parse_unary();
        while let Some(t @ (Token::Asterisk | Token::Slash)) = self.tokens.peek().cloned() {
            let op = match t {
                Token::Asterisk => Op::Mul,
                Token::Slash => Op::Div,
                _ => unreachable!(),
            };
            self.tokens.next();
            let right = self.parse_unary();
            left = Expr::BinOp(Box::new(left), op, Box::new(right));
        }
        left
    }
    fn parse_unary(&mut self) -> Expr {
        if let Some(t) = self.tokens.peek().cloned() {
            match t {
                Token::Plus => {
                    self.tokens.next();
                    let expr = self.parse_primary();
                    Expr::UnaryOp(Op::Add, Box::new(expr))
                }
                Token::Minus => {
                    self.tokens.next();
                    let expr = self.parse_primary();
                    Expr::UnaryOp(Op::Sub, Box::new(expr))
                }
                _ => self.parse_primary(),
            }
        } else {
            panic!("unexpected behaviour");
        }
    }
    fn parse_primary(&mut self) -> Expr {
        if let Some(t) = self.tokens.next() {
            let tok = match t {
                Token::Number(n) => Expr::Num(n),
                Token::Ident(x) => Expr::Ident(x),
                Token::LParen => {
                    let expr = self.parse_expr();
                    self.expect(Token::RParen);
                    expr
                }
                t => panic!("unexpected token in expression: {:?}", t),
            };
            tok
        } else {
            panic!("unexpected behaviour");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_expr() {
        use crate::lexer::Lexer;
        let mut lexer = Lexer::new("1 + 2 * 3");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr();
        match expr {
            Expr::BinOp(left, Op::Add, right) => {
                match *left {
                    Expr::Num(1) => {}
                    _ => panic!("expected Num(1)"),
                }
                match *right {
                    Expr::BinOp(inner_left, Op::Mul, inner_right) => {
                        match *inner_left {
                            Expr::Num(2) => {}
                            _ => panic!("expected Num(2)"),
                        }
                        match *inner_right {
                            Expr::Num(3) => {}
                            _ => panic!("expected Num(3)"),
                        }
                    }
                    _ => panic!("expected BinOp with Mul"),
                }
            }
            _ => panic!("expected BinOp with Add"),
        }
    }
}
