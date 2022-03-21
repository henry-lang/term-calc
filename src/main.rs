use std::io;
use std::iter::Peekable;
use std::slice::Iter;

#[derive(PartialEq, Debug)]
enum TokenType {
    PlusSign,
    MinusSign,
    TimesSign,
    DivideSign,
    
    OpenParen,
    CloseParen,

    Number
}

use TokenType::*;

#[derive(PartialEq, Debug)]
struct Token {
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

enum UnaryOpType {
    Negate
}

enum BinaryOpType {
    Add,
    Subtract,
    Multiply,
    Divide
}


type ChildNode = Box<Node>;

enum Node {
    Number(f64),

    UnaryOp {
        op_type: UnaryOpType,
        operand: ChildNode 
    },
    
    BinaryOp {
        op_type: BinaryOpType,
        lhs: ChildNode,
        rhs: ChildNode
    }
}

// type ParserIterator<'a> = Peekable<Iter<'a, Token>>;

struct Parser<'a> {
    index: usize,
    current_token: &'a Token,
    tokens: &'a Vec<Token>
}
    
impl<'a> Parser<'a> {
    fn new(tokens: &'a Vec<Token>) -> Self {
        Self {index: 0, tokens, current_token: &tokens[0]}
    }
    
    fn consume(&mut self, token_type: Token) {
        if token_type.token_type == self.current_token.token_type {
            println!("pass")
        } else {
            panic!("\x1b[31mExpected {:?}, found {:?}\x1b[0m", token_type, *self.current_token)
        }
    }
    
    fn get_primary(&self) {
        
    }

    fn get_term(&mut self) {
        
    }

    fn get_expression(&mut self) {
        /*
        expr: term ((+ | -) term)*
        term: factor ((* | /) factor)*
        factor: NUMBER |
                OPENPAREN expr CLOSEPAREN |
                (* | -) factor
        

        let mut  = self.get_term();
        while [PlusSign, MinusSign].contains(&self.current_token.token_type) {
            println!("add")
        }
        */
    }
}

fn tokenize(expression: String) -> Option<Vec<Token>> {
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
            
            tokens.push(Token::new_with_value(Number, value));
        } else {
            tokens.push(match c {
                '+' => Token::new(PlusSign),
                '-' => Token::new(MinusSign),
                '*' => Token::new(TimesSign),
                '/' => Token::new(DivideSign),
                '(' => Token::new(OpenParen),
                ')' => Token::new(CloseParen),
                _ => continue
            });
        }
    }
    
    Some(tokens)
}
    
fn main() -> io::Result<()> {
    let expression = "3 * (4 + 2)".to_string();
    let tokens = tokenize(expression).unwrap();
    println!("{:?}", tokens);
    let mut parser = Parser::new(&tokens);
    parser.get_expression();
    
    Ok(())
} 
