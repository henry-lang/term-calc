#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TokenType {
    PlusSign,
    MinusSign,
    TimesSign,
    DivideSign,
    PowerSign,
    
    OpenParen,
    CloseParen,

    NumLiteral
}

use TokenType::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: Option<f64>
}

impl Token {
    fn new(token_type: TokenType) -> Self {
        Self {token_type, value: None}
    }

    fn new_with_value(token_type: TokenType, value: f64) -> Self {
        Self {token_type, value: Some(value)}
    }
}

pub fn tokenize(expression: &String) -> Option<Vec<Token>> {
    let filtered = expression.split_whitespace().collect::<String>();
    let mut tokens = Vec::<Token>::new();
    
    let mut iter = filtered.chars().enumerate().peekable();
    while let Some((idx, c)) = iter.next() {
        if c.is_numeric() {
            let mut end_idx = idx + 1;
            while let Some(_) = iter.next_if(|(_, next)| next.is_numeric() || *next == '.') {
                end_idx += 1;
            }
    
            let value = filtered[idx..end_idx]
                .parse::<f64>()
                .expect("number is valid");
            
            tokens.push(Token::new_with_value(NumLiteral, value));
        } else {
            tokens.push(match c {
                '+' => Token::new(PlusSign),
                '-' => Token::new(MinusSign),
                '*' => if let Some(_) = iter.next_if(|(_, next)| *next == c) { Token::new(PowerSign) } else { Token::new(TimesSign) }
                '/' => Token::new(DivideSign),
                '^' => Token::new(PowerSign),
                '(' => Token::new(OpenParen),
                ')' => Token::new(CloseParen),
                _ => continue
            });
        }
    }
    
    Some(tokens)
 }
