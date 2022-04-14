use std::{
    io::{self, Write},
    path::PathBuf,
};

use calculate::{calculate, Config, Identifiers};

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
