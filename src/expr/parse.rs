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

pub fn parse(_tokens: &[Token]) -> Result<SyntaxTree, ParseError> {
    Ok(SyntaxTree::Num(5))
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
