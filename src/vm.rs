use crate::enums::Instr;

pub fn run(code: &[Instr]) -> Result<f64, String> {
    let mut stack: Vec<f64> = Vec::new();

    for ins in code {
        match ins {
            Instr::Push(n) => stack.push(*n),
            Instr::Neg => {
                let a = stack.pop().ok_or("Stack is empty")?;
                stack.push(-a);
            }
            Instr::Add => {
                let b = stack.pop().ok_or("Stack is empty")?;
                let a = stack.pop().ok_or("Stack is empty")?;
                stack.push(a + b);
            }
            Instr::Sub => {
                let b = stack.pop().ok_or("Stack is empty")?;
                let a = stack.pop().ok_or("Stack is empty")?;
                stack.push(a - b);
            }
            Instr::Mul => {
                let b = stack.pop().ok_or("Stack is empty")?;
                let a = stack.pop().ok_or("Stack is empty")?;
                stack.push(a * b);
            }
            Instr::Div => {
                let b = stack.pop().ok_or("Stack is empty")?;
                if b == 0.0 {
                    return Err("Division by zero".into());
                }

                let a = stack.pop().ok_or("Stack is empty")?;
                stack.push(a / b);
            }
            Instr::Exp => {
                let b = stack.pop().ok_or("Stack is empty")?;
                let a = stack.pop().ok_or("Stack is empty")?;
                stack.push(a.powf(b));
            }
        }
    }

    stack.pop().ok_or("Stack is empty".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{codegen::codegen, lexer::lex, parser::parse};

    fn calc(input: &str) -> Result<f64, String> {
        let tokens = lex(input)?;
        let expr = parse(tokens)?;
        let mut instructions = Vec::new();
        codegen(&expr, &mut instructions);
        run(&instructions)
    }

    #[test]
    fn test_run_simple_expression() {
        let result = calc("1 + 2 * 3").unwrap();
        assert_eq!(result, 7.0);
    }

    #[test]
    fn test_run_expression_with_parentheses() {
        let input = "(1 + 2) * 3";
        let result = calc(input).unwrap();
        assert_eq!(result, 9.0);
    }

    #[test]
    fn test_run_expression_with_subtraction() {
        let input = "10 - 4 - 3";
        let result = calc(input).unwrap();
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_run_expression_with_negation() {
        let input = "-2 * 3";
        let result = calc(input).unwrap();
        assert_eq!(result, -6.0);
    }

    #[test]
    fn test_run_division_by_zero() {
        let input = "1 / 0";
        let result = calc(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_run_exponent_is_right_associative() {
        let result = calc("2^3^2").unwrap();
        assert_eq!(result, 512.0);
    }
}
