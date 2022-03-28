mod config;
mod identifiers;
mod parser;
mod tokenizer;
mod traverse;

use config::Config;

use identifiers::Identifiers;
use parser::Parser;
use tokenizer::tokenize;
use traverse::traverse;

use std::{
    io::{self, Write},
    path::PathBuf,
};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let config = Config::load(
        [
            home::home_dir().expect("home directory"),
            "term-calc.config.toml".into(),
        ]
        .iter()
        .collect::<PathBuf>(),
    );

    let identifiers = Identifiers::get();

    loop {
        print!("calc > ");
        io::stdout().flush()?;
        let mut expression = String::new();

        stdin.read_line(&mut expression)?;
        match expression.trim() {
            "exit" => break,
            _ => (),
        }

        let tokens = tokenize(&expression).unwrap();

        if config.debug {
            println!("Tokens: {:?}", tokens);
        }

        if tokens.is_empty() {
            continue;
        }

        let mut parser = Parser::new(tokens, &identifiers);
        let node = parser.get_expression();

        if config.debug {
            println!("Node Tree: {:?}", node);
        }

        let value = traverse(node);
        println!("{}", value);
    }

    Ok(())
}
