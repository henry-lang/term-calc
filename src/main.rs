mod parser;
mod tokenizer;
mod traverse;
mod functions;

use crate::parser::Parser;
use crate::tokenizer::tokenize;
use crate::traverse::traverse;
use crate::functions::get_registry;

use std::io::{self, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let mut debug = false;
    let registry = get_registry();

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

        let mut parser = Parser::new(tokens, &registry);
        let node = parser.get_expression();

        if debug {
            println!("Node Tree: {:?}", node);
        }

        let value = traverse(node);
        println!("{}", value);
    }

    Ok(())
}
