use crate::enums::Token;

pub fn lex(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            c if c.is_whitespace() => {
                chars.next();
            }
            c if c.is_ascii_digit() => {
                let mut num_str = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() || c == '.' {
                        num_str.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let num = num_str.parse::<f64>().map_err(|e| e.to_string())?;
                tokens.push(Token::Num(num));
            }
            '+' => push_token(&mut tokens, Token::Plus, &mut chars),
            '-' => push_token(&mut tokens, Token::Minus, &mut chars),
            '*' => push_token(&mut tokens, Token::Star, &mut chars),
            '/' => push_token(&mut tokens, Token::Slash, &mut chars),
            '^' => push_token(&mut tokens, Token::Exp, &mut chars),
            '(' => push_token(&mut tokens, Token::LParen, &mut chars),
            ')' => push_token(&mut tokens, Token::RParen, &mut chars),
            _ => return Err(format!("Unexpected character: {}", c)),
        }
    }

    tokens.push(Token::Eof);
    Ok(tokens)
}

fn push_token(
    tokens: &mut Vec<Token>,
    token: Token,
    characters: &mut std::iter::Peekable<std::str::Chars>,
) {
    tokens.push(token);
    characters.next();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex_simple_expression() {
        let input = "1 + 2";
        let expected_tokens = vec![Token::Num(1.0), Token::Plus, Token::Num(2.0), Token::Eof];

        let result = lex(input).unwrap();
        assert_eq!(result, expected_tokens);
    }

    #[test]
    fn test_lex_seven() {
        let input = "  7  ";
        let expected_tokens = vec![Token::Num(7.0), Token::Eof];

        let result = lex(input).unwrap();
        assert_eq!(result, expected_tokens);
    }

    #[test]
    fn test_lex_two_point_four() {
        let input = "2.4";
        let expected_tokens = vec![Token::Num(2.4), Token::Eof];

        let result = lex(input).unwrap();
        assert_eq!(result, expected_tokens);
    }

    #[test]
    fn test_lex_two_dollar_sign_three() {
        let input = "2$3";
        let result = lex(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_one_point_two_point_three() {
        let input = "1.2.3";
        let result = lex(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_lex_with_exponentiation() {
        let input = "2 ^ 3";
        let expected_tokens = vec![Token::Num(2.0), Token::Exp, Token::Num(3.0), Token::Eof];
        assert!(lex(input).unwrap() == expected_tokens);
    }
}
