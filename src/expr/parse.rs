use core::slice::Iter;

use super::lex::Token;
use super::Expr;
use super::Operator;
use super::ParseError;

pub fn parse(tokens: &[Token]) -> Result<Expr, ParseError> {
    parse_it(&mut tokens.iter(), false)
}

fn parse_it(it: &mut Iter<Token>, expect_close_paren: bool) -> Result<Expr, ParseError> {
    let mut result = parse_one(it)?;
    let mut maybe_op = parse_operator(it, expect_close_paren)?;
    while let Some(op) = maybe_op {
        let rhs = parse_one(it)?;
        result = Expr::Op {
            op,
            left: Box::new(result),
            right: Box::new(rhs),
        };

        maybe_op = parse_operator(it, expect_close_paren)?;
    }

    Ok(result)
}

fn parse_one(it: &mut Iter<Token>) -> Result<Expr, ParseError> {
    let t = it.next().ok_or(ParseError::UnexpectedEnd)?;
    match t {
        Token::OpenParen => parse_it(it, true),
        Token::Num(x) => Ok(Expr::Num(*x)),
        x => Err(ParseError::UnexpectedToken(*x)),
    }
}

fn parse_operator(
    it: &mut Iter<Token>,
    expect_close_paren: bool,
) -> Result<Option<Operator>, ParseError> {
    match it.next() {
        None if !expect_close_paren => Ok(None),
        None => Err(ParseError::UnexpectedEnd),
        Some(Token::Op(op)) => Ok(Some(*op)),
        Some(Token::CloseParen) if expect_close_paren => Ok(None),
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
            Ok(Expr::Op {
                op: Operator::Add,
                left: Box::new(Expr::Num(5)),
                right: Box::new(Expr::Num(2))
            }),
            parse(&input[..])
        );
    }

    #[test]
    fn unmatched_close_paren() {
        let input = vec![Token::Num(3), Token::CloseParen];
        assert_eq!(
            Err(ParseError::UnexpectedToken(Token::CloseParen)),
            parse(&input[..])
        );
    }

    #[test]
    fn unmatched_open_paren() {
        let input = vec![Token::OpenParen, Token::Num(3)];
        assert_eq!(Err(ParseError::UnexpectedEnd), parse(&input[..]));
    }
}
