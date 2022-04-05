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

    let identifiers = Identifiers::get(&config);

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

#[cfg(test)]
mod tests {
    use crate::Config;
    use crate::Identifiers;
    use std::f64;

    fn run_default(expression: &str) -> Result<f64, String> {
        crate::calculate(
            expression,
            &Identifiers::get(&Config::default()),
            &Config::default(),
        )
    }

    #[test]
    fn decimals() {
        assert_eq!(run_default("0.5").unwrap(), 0.5);
    }

    #[test]
    #[should_panic]
    fn invalid_number() {
        run_default("3132193.21933.213921.3").unwrap();
    }

    #[test]
    fn basic_expressions() {
        assert_eq!(run_default("1 + 2 - 3 - 4").unwrap(), -4.0);
    }

    #[test]
    fn mixed_expressions() {
        assert_eq!(run_default("5 * (3 + 4) ^ 2").unwrap(), 245.0);
    }

    #[test]
    fn identifiers() {
        assert_eq!(run_default("pi").unwrap(), f64::consts::PI);
    }

    #[test]
    fn functions() {
        assert_eq!(run_default("sin(2 * 5)").unwrap(), f64::sin(10.0));
    }
}
