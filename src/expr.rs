use std::collections::HashMap;

use num_rational::Rational64;

/// The basic (binary) operators which are allowed in the dice game.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Add => write!(f, "Addition"),
            Operator::Sub => write!(f, "Subtraction"),
            Operator::Mul => write!(f, "Multiplication"),
            Operator::Div => write!(f, "Division"),
        }
    }
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

mod error;

pub use error::{EvalError, ParseError};

mod lex;
mod parse;

impl Expr {
    pub fn parse(s: &str) -> Result<Expr, ParseError> {
        parse::parse(lex::lex(s)?.as_slice())
    }

    pub fn histogram(&self) -> HashMap<u32, u32> {
        fn inner(e: &Expr, out: &mut HashMap<u32, u32>) {
            match e {
                Expr::Num(x) => *out.entry(*x).or_insert(0) += 1,
                Expr::Op { op: _, left, right } => {
                    inner(&left, out);
                    inner(&right, out);
                }
            }
        }

        let mut res = HashMap::new();
        inner(self, &mut res);
        res
    }

    pub fn eval(&self) -> Result<Rational64, EvalError> {
        match self {
            Expr::Num(x) => Ok(Rational64::from_integer(i64::from(*x))),
            Expr::Op { op, left, right } => {
                let lhs = left.eval()?;
                let rhs = right.eval()?;
                match op {
                    Operator::Div if rhs == Rational64::from_integer(0) => {
                        Err(EvalError::DivisionByZero)
                    }
                    Operator::Div => Ok(lhs / rhs),
                    Operator::Add => Ok(lhs + rhs),
                    Operator::Mul => Ok(lhs * rhs),
                    Operator::Sub => Ok(lhs - rhs),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_histogram() {
        let e = Expr::Op {
            op: Operator::Add,
            left: Box::new(Expr::Num(6)),
            right: Box::new(Expr::Op {
                op: Operator::Div,
                left: Box::new(Expr::Num(5)),
                right: Box::new(Expr::Num(6)),
            }),
        };

        let mut expected = HashMap::new();
        expected.insert(6, 2);
        expected.insert(5, 1);

        assert_eq!(expected, e.histogram());
    }

    #[test]
    fn basic_eval() {
        let e = Expr::Op {
            op: Operator::Add,
            left: Box::new(Expr::Num(6)),
            right: Box::new(Expr::Op {
                op: Operator::Div,
                left: Box::new(Expr::Num(5)),
                right: Box::new(Expr::Num(6)),
            }),
        };
        assert_eq!(Ok(Rational64::new(41, 6)), e.eval());
    }

    #[test]
    fn eval_div_by_zero() {
        let e = Expr::Op {
            op: Operator::Div,
            left: Box::new(Expr::Num(1)),
            right: Box::new(Expr::Op {
                op: Operator::Sub,
                left: Box::new(Expr::Num(1)),
                right: Box::new(Expr::Num(1)),
            }),
        };
        assert_eq!(Err(EvalError::DivisionByZero), e.eval());
    }
}
