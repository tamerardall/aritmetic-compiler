#[derive(Debug, PartialEq)]
pub enum Token {
    Num(f64),
    Plus,
    Minus,
    Star,
    Slash,
    Exp,
    LParen,
    RParen,
    Eof,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Num(f64),
    Neg(Box<Expr>),
    Bin(Box<Expr>, Operation, Box<Expr>),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    Exp,
}

#[derive(Debug, PartialEq)]
pub enum Instr {
    Push(f64),
    Add,
    Sub,
    Mul,
    Div,
    Exp,
    Neg,
}
