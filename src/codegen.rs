use crate::enums::{Expr, Instr, Operation};

pub fn codegen(e: &Expr, out: &mut Vec<Instr>) {
    match e {
        Expr::Num(n) => out.push(Instr::Push(*n)),
        Expr::Neg(inner) => {
            codegen(inner, out);
            out.push(Instr::Neg);
        }
        Expr::Bin(l, op, r) => {
            codegen(l, out);
            codegen(r, out);
            out.push(match op {
                Operation::Add => Instr::Add,
                Operation::Sub => Instr::Sub,
                Operation::Mul => Instr::Mul,
                Operation::Div => Instr::Div,
                Operation::Exp => Instr::Exp,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_codegen_simple_expression() {
        let expr = Expr::Bin(
            Box::new(Expr::Num(1.0)),
            Operation::Add,
            Box::new(Expr::Bin(
                Box::new(Expr::Num(2.0)),
                Operation::Mul,
                Box::new(Expr::Num(3.0)),
            )),
        );

        let mut instructions = Vec::new();
        codegen(&expr, &mut instructions);

        let expected_instructions = vec![
            Instr::Push(1.0),
            Instr::Push(2.0),
            Instr::Push(3.0),
            Instr::Mul,
            Instr::Add,
        ];

        assert_eq!(instructions, expected_instructions);
    }

    #[test]
    fn test_codegen_expression_with_parentheses() {
        let expr = Expr::Bin(
            Box::new(Expr::Bin(
                Box::new(Expr::Num(1.0)),
                Operation::Add,
                Box::new(Expr::Num(2.0)),
            )),
            Operation::Mul,
            Box::new(Expr::Num(3.0)),
        );

        let mut instructions = Vec::new();
        codegen(&expr, &mut instructions);

        let expected_instructions = vec![
            Instr::Push(1.0),
            Instr::Push(2.0),
            Instr::Add,
            Instr::Push(3.0),
            Instr::Mul,
        ];

        assert_eq!(instructions, expected_instructions);
    }

    #[test]
    fn test_codegen_negation() {
        let expr = Expr::Neg(Box::new(Expr::Num(5.0)));

        let mut instructions = Vec::new();
        codegen(&expr, &mut instructions);

        let expected_instructions = vec![Instr::Push(5.0), Instr::Neg];

        assert_eq!(instructions, expected_instructions);
    }
}
