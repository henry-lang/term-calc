mod tokenizer;
mod parser;

use crate::tokenizer::tokenize;
use crate::parser::Parser;

use std::io::{self, Write};
    
fn main() -> io::Result<()> {
    let stdin = io::stdin();

    loop {
        print!("calc > ");
        io::stdout().flush()?;
        let mut expression: String = String::new();

        stdin.read_line(&mut expression)?;
        if expression.trim() == "exit" {
            break
        }

        let tokens = tokenize(&expression).unwrap();
        println!("{:?}", tokens);
        if tokens.is_empty() {
            continue
        }

        let mut parser = Parser::new(tokens);
        println!("{:?}", parser.get_expression());
    }

    Ok(())
} 
