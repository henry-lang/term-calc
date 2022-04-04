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

pub fn calculate(
    expression: &str,
    identifiers: &Identifiers,
    config: &Config,
) -> Result<f64, String> {
    let tokens = tokenize(expression);

    match tokens {
        Ok(tokens) => {
            if config.debug {
                dbg!(&tokens);
            }

            let mut parser = Parser::new(tokens, identifiers);
            let node = parser.get_expression();
            if config.debug {
                println!("{:?}", node);
            }

            let value = traverse(node);
            Ok(value)
        }
        Err(msg) => Err(msg),
    }
}

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
        if expression.trim() == "exit" {
            break;
        }
        match calculate(&expression, &identifiers, &config) {
            Ok(value) => println!("{}", value),
            Err(msg) => println!("Error: {}", msg),
        }
    }

    Ok(())
}
