use core::slice::Iter;

use super::lex::Token;
use super::Operator;
use super::ParseError;

#[derive(PartialEq, Eq, Debug)]
pub enum SyntaxTree {
    Num(u32),
    Op {
        op: Operator,
        left: Box<SyntaxTree>,
        right: Box<SyntaxTree>,
    },
}

pub fn parse(tokens: &[Token]) -> Result<SyntaxTree, ParseError> {
    parse_it(&mut tokens.iter())
}

fn parse_it(it: &mut Iter<Token>) -> Result<SyntaxTree, ParseError> {
    let mut result = parse_one(it)?;
    let mut op = parse_operator(it)?;
    while op.is_some() {
        let rhs = parse_one(it)?;
        result = SyntaxTree::Op {
            op: op.unwrap(),
            left: Box::new(result),
            right: Box::new(rhs),
        };

        op = parse_operator(it)?;
    }

    Ok(result)
}

fn parse_one(it: &mut Iter<Token>) -> Result<SyntaxTree, ParseError> {
    let t = it.next().ok_or(ParseError::UnexpectedEnd)?;
    match t {
        Token::OpenParen => parse_it(it),
        Token::Num(x) => Ok(SyntaxTree::Num(*x)),
        x => Err(ParseError::UnexpectedToken(*x)),
    }
}

fn parse_operator(it: &mut Iter<Token>) -> Result<Option<Operator>, ParseError> {
    match it.next() {
        None => Ok(None),
        Some(Token::Op(op)) => Ok(Some(*op)),
        Some(Token::CloseParen) => Ok(None),
        Some(x) => Err(ParseError::UnexpectedToken(*x)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let input = vec![
            Token::OpenParen,
            Token::Num(5),
            Token::Op(Operator::Add),
            Token::Num(2),
            Token::CloseParen,
        ];
        assert_eq!(
            SyntaxTree::Op {
                op: Operator::Add,
                left: Box::new(SyntaxTree::Num(5)),
                right: Box::new(SyntaxTree::Num(2))
            },
            parse(&input[..]).unwrap()
        );
    }
}
