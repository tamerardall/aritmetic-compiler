# aritmetic-compiler

A small compiler written in Rust that turns arithmetic expressions into bytecode and runs it. Built to learn how compilers work. No external dependencies.

```
> 1 + 2 * 3
7
> (1 + 2) * 3
9
> 2^3^2
512
> -2^2
-4
> 1 / 0
Err: Division by zero
```

## How it works

An expression goes through four stages:

```
"1 + 2 * 3"
   │  lexer.rs     splits the text into tokens
   ▼
[Num(1) Plus Num(2) Star Num(3) Eof]
   │  parser.rs    builds a syntax tree (AST) with a Pratt parser
   ▼
Bin(1, Add, Bin(2, Mul, 3))
   │  codegen.rs   walks the tree bottom-up and emits instructions
   ▼
[Push(1), Push(2), Push(3), Mul, Add]
   │  vm.rs        runs them on a stack-based virtual machine
   ▼
7
```

| File | Role |
|---|---|
| `src/lexer.rs` | Turns characters into `Token`s |
| `src/parser.rs` | Builds an `Expr` tree from the tokens and resolves operator precedence |
| `src/codegen.rs` | Generates a list of `Instr` from the `Expr` tree |
| `src/vm.rs` | Executes the instructions on a stack |
| `src/enums.rs` | Definitions of `Token`, `Expr`, `Operation` and `Instr` |
| `src/main.rs` | REPL that reads lines from the console |

## Supported syntax

| Operator | Meaning | Precedence | Associativity |
|---|---|---|---|
| `^` | exponentiation | highest | right to left |
| `-x` | unary minus | lower than `^` | |
| `*` `/` | multiplication, division | | left to right |
| `+` `-` | addition, subtraction | lowest | left to right |

Parentheses and decimal numbers (`2.5`) also work. The decimal separator is a dot.

Errors are returned as messages instead of crashing: unexpected character, missing closing parenthesis, extra token, empty stack, division by zero.

## Usage

Requires Rust 1.85 or newer (edition 2024).

```
cargo run      # starts the REPL, press Ctrl+D to exit
cargo test     # runs the 20 unit tests
```

## Limitations

- Numbers are `f64`. Integers stay exact up to about 9 × 10^15, larger results are rounded.
- No variables or functions.
- Error messages do not include a position.
