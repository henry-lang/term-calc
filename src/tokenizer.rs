#[derive(Clone, PartialEq, Debug)]
pub enum Token {
    PlusSign,
    MinusSign,
    TimesSign,
    DivideSign,
    PowerSign,

    OpenParen,
    CloseParen,

    NumLiteral(f64),
    NameLiteral(String), // Function name
}

use Token::*;

pub fn tokenize(expression: &str) -> Option<Vec<Token>> {
    let filtered = expression.split_whitespace().collect::<String>();
    let mut tokens = Vec::<Token>::new();

    let mut iter = filtered.chars().enumerate().peekable();
    while let Some((idx, c)) = iter.next() {
        tokens.push(match c {
            '+' => PlusSign,
            '-' => MinusSign,
            '*' => {
                if iter.next_if(|(_, next)| *next == c).is_some() {
                    PowerSign
                } else {
                    TimesSign
                }
            }
            '/' => DivideSign,
            '^' => PowerSign,
            '(' => OpenParen,
            ')' => CloseParen,
            _ => {
                if c.is_numeric() {
                    let mut end_idx = idx + 1;
                    while iter
                        .next_if(|(_, next)| next.is_numeric() || *next == '.')
                        .is_some()
                    {
                        end_idx += 1;
                    }

                    let value = filtered[idx..end_idx]
                        .parse::<f64>()
                        .expect("number is valid");

                    NumLiteral(value)
                } else if c.is_alphabetic() {
                    let mut end_idx = idx + 1;
                    while iter
                        .next_if(|(_, next)| next.is_alphabetic() || *next == '_')
                        .is_some()
                    {
                        end_idx += 1;
                    }

                    let value = filtered[idx..end_idx].to_string();

                    NameLiteral(value)
                } else {
                    panic!("Unexpected character {}", c);
                }
            }
        });
    }

    Some(tokens)
}
