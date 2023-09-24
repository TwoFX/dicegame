#[derive(PartialEq, Eq, Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Token {
    Num(u32),
    OpenParen,
    CloseParen,
    Op(Operator),
}

#[derive(PartialEq, Eq, Debug)]
pub enum Error {
    UnknownToken(char),
    NumberTooLarge,
}

use std::{iter::Peekable, str::Chars};

use Error::*;
use Operator::*;
use Token::*;

pub fn lex(input: &str) -> Result<Vec<Token>, Error> {
    lex_chars(input.chars().peekable())
}

fn lex_chars(mut it: Peekable<Chars>) -> Result<Vec<Token>, Error> {
    let mut result = Vec::new();
    let mut current = it.next();
    while current.is_some() {
        let c = current.unwrap();
        if c == '(' {
            result.push(OpenParen);
        } else if c == ')' {
            result.push(CloseParen);
        } else if c == '+' {
            result.push(Op(Add))
        } else if c == '-' {
            result.push(Op(Sub))
        } else if c == '/' {
            result.push(Op(Div))
        } else if c == '*' {
            result.push(Op(Mul))
        } else if c.is_whitespace() {
            // Skip
        } else if c.is_ascii_digit() {
            result.push(parse_number(&mut it, c)?);
        } else {
            return Err(UnknownToken(c));
        }

        current = it.next()
    }
    Ok(result)
}

fn parse_number(it: &mut Peekable<Chars>, first_digit: char) -> Result<Token, Error> {
    let mut result = first_digit.to_digit(10).unwrap();
    while it.peek().map(|d| d.is_digit(10)).unwrap_or(false) {
        let intermediate = match result.checked_mul(10) {
            Some(r) => r,
            None => return Err(NumberTooLarge),
        };
        let next_digit = it.next().unwrap().to_digit(10).unwrap();
        result = match intermediate.checked_add(next_digit) {
            Some(r) => r,
            None => return Err(NumberTooLarge),
        }
    }
    Ok(Num(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let input = "(34-   2) )+";
        assert_eq!(
            vec![
                OpenParen,
                Num(34),
                Op(Sub),
                Num(2),
                CloseParen,
                CloseParen,
                Op(Add)
            ],
            lex(input).unwrap()
        );
    }

    #[test]
    fn unknown_token() {
        let input = "((!?34";
        assert_eq!(UnknownToken('!'), lex(input).unwrap_err());
    }

    #[test]
    fn large_number() {
        let input = "1000000000000";
        assert_eq!(NumberTooLarge, lex(input).unwrap_err());
    }
}
