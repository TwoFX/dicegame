use super::lex::Token;
use thiserror::Error;

#[non_exhaustive]
#[derive(Error, PartialEq, Eq, Debug)]
pub enum ParseError {
    #[error("unknown token '{0}'")]
    UnknownToken(char),
    #[error("number too large")]
    NumberTooLarge,
    #[error("unexpected token '{0}'")]
    UnexpectedToken(Token),
    #[error("unexpected end of input")]
    UnexpectedEnd,
    #[error("unexpected structure")]
    UnexpectedStructure,
}

#[non_exhaustive]
#[derive(Error, PartialEq, Eq, Debug)]
pub enum EvalError {
    #[error("division by zero")]
    DivisionByZero,
}
