/// The basic (binary) operators which are allowed in the dice game.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
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

pub use parse::SyntaxTree;

pub fn parse(s: &str) -> Result<SyntaxTree, ParseError> {
    parse::parse(&lex::lex(s)?[..])
}
