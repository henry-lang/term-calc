mod parser;
mod tokenizer;
mod traverse;

use crate::parser::Parser;
use crate::tokenizer::tokenize;
use crate::traverse::traverse;

use std::io::{self, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let mut debug = false;

    loop {
        print!("calc > ");
        io::stdout().flush()?;
        let mut expression: String = String::new();

        stdin.read_line(&mut expression)?;
        match expression.trim() {
            "exit" => break,
            "debug" => {
                debug = !debug;
                println!("debug mode {}", debug);
                continue;
            }
            _ => (),
        }

        let tokens = tokenize(&expression).unwrap();

        if debug {
            println!("Tokens: {:?}", tokens);
        }

        if tokens.is_empty() {
            continue;
        }

        let mut parser = Parser::new(tokens);
        let node = parser.get_expression();

        if debug {
            println!("Node Tree: {:?}", node);
        }

        let value = traverse(node);
        println!("{}", value);
    }

    Ok(())
}
