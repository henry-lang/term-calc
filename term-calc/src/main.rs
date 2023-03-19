use std::{
    io::{self, Write},
    path::PathBuf,
};

use ansi_term::Color::{Red, White};

use calculate::{calculate, Config, Context};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let config = Config::load(
        [
            home::home_dir().expect("home directory"),
            ".term-calc.toml".into(),
        ]
        .iter()
        .collect::<PathBuf>(),
    );

    let identifiers = Context::create(&config);

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
            Err(msg) => println!(
                "{}{}{}",
                Red.bold().paint("error"),
                White.bold().paint(": "),
                msg
            ),
        }
    }

    Ok(())
}
