use core::slice::Iter;

use super::lex::Token;
use super::Expr;
use super::Operator;
use super::ParseError;

pub fn parse(tokens: &[Token]) -> Result<Expr, ParseError> {
    parse_it(&mut tokens.iter(), false)
}

#[derive(PartialEq, Eq, Debug)]
enum Intermediate {
    Expr(Expr),
    Op(Operator),
}

fn parse_it(it: &mut Iter<Token>, expect_close_paren: bool) -> Result<Expr, ParseError> {
    parse_from_sequence(parse_to_sequence(it, expect_close_paren)?)
}

fn parse_from_sequence(v: Vec<Intermediate>) -> Result<Expr, ParseError> {
    match split_at_best_location(v)? {
        Split::Single(ex) => Ok(ex),
        Split::Split { lhs, op, rhs } => Ok(Expr::Op {
            op,
            left: Box::new(parse_from_sequence(lhs)?),
            right: Box::new(parse_from_sequence(rhs)?),
        }),
    }
}

#[derive(Debug)]
enum Split {
    Single(Expr),
    Split {
        lhs: Vec<Intermediate>,
        op: Operator,
        rhs: Vec<Intermediate>,
    },
}

fn split_at_best_location(v: Vec<Intermediate>) -> Result<Split, ParseError> {
    let mut best: Option<(usize, Operator)> = None;
    for (i, y) in v.iter().enumerate() {
        if let Intermediate::Op(op) = y {
            if best
                .map(|(_, o)| op.precedence() <= o.precedence())
                .unwrap_or(true)
            {
                best = Some((i, *op));
            }
        }
    }

    match best {
        None => match only(v).ok_or(ParseError::UnexpectedStructure)? {
            Intermediate::Expr(ex) => Ok(Split::Single(ex)),
            _ => Err(ParseError::UnexpectedStructure),
        },
        Some((i, op)) => {
            let mut lhs = v;
            let rhs = lhs.split_off(i + 1);
            lhs.pop();

            Ok(Split::Split { lhs, op, rhs })
        }
    }
}

fn only<T>(v: Vec<T>) -> Option<T> {
    if v.len() != 1 {
        None
    } else {
        v.into_iter().next()
    }
}

fn parse_to_sequence(
    it: &mut Iter<Token>,
    expect_close_paren: bool,
) -> Result<Vec<Intermediate>, ParseError> {
    let mut result = vec![Intermediate::Expr(parse_one(it)?)];
    let mut maybe_op = parse_operator(it, expect_close_paren)?;

    while let Some(op) = maybe_op {
        result.push(Intermediate::Op(op));
        result.push(Intermediate::Expr(parse_one(it)?));

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
            parse(input.as_slice())
        );
    }

    #[test]
    fn unmatched_close_paren() {
        let input = vec![Token::Num(3), Token::CloseParen];
        assert_eq!(
            Err(ParseError::UnexpectedToken(Token::CloseParen)),
            parse(input.as_slice())
        );
    }

    #[test]
    fn unmatched_open_paren() {
        let input = vec![Token::OpenParen, Token::Num(3)];
        assert_eq!(Err(ParseError::UnexpectedEnd), parse(input.as_slice()));
    }

    #[test]
    fn precendence1() {
        let input = vec![
            Token::Num(2),
            Token::Op(Operator::Sub),
            Token::Num(3),
            Token::Op(Operator::Mul),
            Token::Num(5),
        ];
        assert_eq!(
            Ok(Expr::Op {
                op: Operator::Sub,
                left: Box::new(Expr::Num(2)),
                right: Box::new(Expr::Op {
                    op: Operator::Mul,
                    left: Box::new(Expr::Num(3)),
                    right: Box::new(Expr::Num(5))
                })
            }),
            parse(input.as_slice())
        );
    }

    #[test]
    fn precendence2() {
        let input = vec![
            Token::Num(2),
            Token::Op(Operator::Mul),
            Token::Num(3),
            Token::Op(Operator::Sub),
            Token::Num(5),
        ];
        assert_eq!(
            Ok(Expr::Op {
                op: Operator::Sub,
                left: Box::new(Expr::Op {
                    op: Operator::Mul,
                    left: Box::new(Expr::Num(2)),
                    right: Box::new(Expr::Num(3))
                }),
                right: Box::new(Expr::Num(5)),
            }),
            parse(input.as_slice())
        );
    }
}
