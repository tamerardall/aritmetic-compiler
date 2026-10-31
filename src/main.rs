mod codegen;
mod enums;
mod lexer;
mod parser;
mod vm;

use std::io::{self, Write};

fn calculate(input: &str) -> Result<f64, String> {
    let tokens = lexer::lex(input)?;
    let expr = parser::parse(tokens)?;
    let mut instructions = Vec::new();
    codegen::codegen(&expr, &mut instructions);
    vm::run(&instructions)
}

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        let n = io::stdin().read_line(&mut line).unwrap();
        if n == 0 {
            break;
        }
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        match calculate(line) {
            Ok(v) => println!("{}", v),
            Err(e) => println!("Err: {}", e),
        }
    }
}
