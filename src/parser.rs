use crate::enums::{Expr, Operation, Token};

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn parse_expr(&mut self, min_bp: u8) -> Result<Expr, String> {
        let mut lhs = match self.tokens.get(self.pos) {
            Some(Token::Num(n)) => {
                self.pos += 1;
                Expr::Num(*n)
            }
            Some(Token::Minus) => {
                self.pos += 1;
                let rhs = self.parse_expr(5)?;
                Expr::Neg(Box::new(rhs))
            }
            Some(Token::LParen) => {
                self.pos += 1;
                let expr = self.parse_expr(0)?;
                if let Some(Token::RParen) = self.tokens.get(self.pos) {
                    self.pos += 1;
                    expr
                } else {
                    return Err("Expected closing parenthesis".into());
                }
            }
            _ => return Err("Expected number or minus sign".into()),
        };

        loop {
            let (op, (l_bp, r_bp)) = match self.tokens.get(self.pos) {
                Some(Token::Plus) => (Operation::Add, (1, 2)),
                Some(Token::Minus) => (Operation::Sub, (1, 2)),
                Some(Token::Star) => (Operation::Mul, (3, 4)),
                Some(Token::Slash) => (Operation::Div, (3, 4)),
                Some(Token::Exp) => (Operation::Exp, (6, 5)),
                _ => break,
            };

            if l_bp < min_bp {
                break;
            }

            self.pos += 1;

            let rhs = self.parse_expr(r_bp)?;

            lhs = Expr::Bin(Box::new(lhs), op, Box::new(rhs));
        }

        Ok(lhs)
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Expr, String> {
    let mut parser = Parser::new(tokens);
    let expr = parser.parse_expr(0)?;

    if let Some(Token::Eof) = parser.tokens.get(parser.pos) {
        Ok(expr)
    } else {
        Err("Unexpected token".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_one_plus_two() {
        let tokens = vec![Token::Num(1.0), Token::Plus, Token::Num(2.0), Token::Eof];

        let exp_expr = Expr::Bin(
            Box::new(Expr::Num(1.0)),
            Operation::Add,
            Box::new(Expr::Num(2.0)),
        );

        let result = parse(tokens).unwrap();
        assert_eq!(result, exp_expr);
    }

    #[test]
    fn test_parse_one_plus_two_times_three() {
        let tokens = vec![
            Token::Num(1.0),
            Token::Plus,
            Token::Num(2.0),
            Token::Star,
            Token::Num(3.0),
            Token::Eof,
        ];

        let exp_expr = Expr::Bin(
            Box::new(Expr::Num(1.0)),
            Operation::Add,
            Box::new(Expr::Bin(
                Box::new(Expr::Num(2.0)),
                Operation::Mul,
                Box::new(Expr::Num(3.0)),
            )),
        );

        let result = parse(tokens).unwrap();
        assert_eq!(result, exp_expr);
    }

    #[test]
    fn test_parse_ten_minus_four_minus_three() {
        let tokens = vec![
            Token::Num(10.0),
            Token::Minus,
            Token::Num(4.0),
            Token::Minus,
            Token::Num(3.0),
            Token::Eof,
        ];

        let exp_expr = Expr::Bin(
            Box::new(Expr::Bin(
                Box::new(Expr::Num(10.0)),
                Operation::Sub,
                Box::new(Expr::Num(4.0)),
            )),
            Operation::Sub,
            Box::new(Expr::Num(3.0)),
        );

        let result = parse(tokens).unwrap();
        assert_eq!(result, exp_expr);
    }

    #[test]
    fn test_parse_exponent_is_right_associative() {
        let tokens = vec![
            Token::Num(2.0),
            Token::Exp,
            Token::Num(3.0),
            Token::Exp,
            Token::Num(2.0),
            Token::Eof,
        ];

        let exp_expr = Expr::Bin(
            Box::new(Expr::Num(2.0)),
            Operation::Exp,
            Box::new(Expr::Bin(
                Box::new(Expr::Num(3.0)),
                Operation::Exp,
                Box::new(Expr::Num(2.0)),
            )),
        );

        let result = parse(tokens).unwrap();
        assert_eq!(result, exp_expr);
    }

    #[test]
    fn test_parse_negation_binds_weaker_than_exponent() {
        let tokens = vec![
            Token::Minus,
            Token::Num(2.0),
            Token::Exp,
            Token::Num(2.0),
            Token::Eof,
        ];

        let exp_expr = Expr::Neg(Box::new(Expr::Bin(
            Box::new(Expr::Num(2.0)),
            Operation::Exp,
            Box::new(Expr::Num(2.0)),
        )));

        let result = parse(tokens).unwrap();
        assert_eq!(result, exp_expr);
    }
}
