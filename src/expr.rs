use std::collections::HashMap;

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

impl Expr {
    pub fn parse(s: &str) -> Result<Expr, ParseError> {
        parse::parse(&lex::lex(s)?[..])
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
}
