/// The basic (binary) operators which are allowed in the dice game.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Expr {
    Num(u32),
    Op {
        op: Operator,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

#[derive(PartialEq, Eq, Debug)]
pub enum ParseError {
    UnknownToken(char),
    NumberTooLarge,
    UnexpectedToken(lex::Token),
    UnexpectedEnd,
}

mod lex;
mod parse;

pub fn parse(s: &str) -> Result<Expr, ParseError> {
    parse::parse(&lex::lex(s)?[..])
}
