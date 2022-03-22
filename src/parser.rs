use crate::tokenizer::{Token, TokenType};
use TokenType::*;

/*
GRAMMAR:
expr: term ((+ | -) term)*
term: power ((* | /) power)*
power: factor ((^) factor)*
factor: NUMBER |
        OPENPAREN expr CLOSEPAREN |
        (-) factor
*/

#[derive(Debug)]
pub enum UnaryOpType {
    Negate,
}

#[derive(Debug)]
pub enum BinaryOpType {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}

type ChildNode = Box<Node>;

#[derive(Debug)]
pub enum Node {
    Number(f64),

    UnaryOp {
        op_type: UnaryOpType,
        operand: ChildNode,
    },

    BinaryOp {
        op_type: BinaryOpType,
        lhs: ChildNode,
        rhs: ChildNode,
    },
}

pub struct Parser {
    current: usize,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { current: 0, tokens }
    }

    pub fn token(&self) -> Token {
        self.tokens[self.current]
    }

    fn consume(&mut self, token_type: TokenType) {
        if token_type == self.token().token_type {
            if self.current + 1 < self.tokens.len() {
                self.current += 1
            }
        } else {
            panic!(
                "\x1b[31mExpected {:?}, found {:?}\x1b[0m",
                token_type,
                self.token()
            );
        }
    }

    fn get_factor(&mut self) -> Box<Node> {
        let token_type = self.token().token_type;
        match token_type {
            NumLiteral => {
                let value = self.token().value.unwrap();
                self.consume(NumLiteral);

                Box::new(Node::Number(value))
            }

            OpenParen => {
                self.consume(OpenParen);
                let expression = self.get_expression();
                self.consume(CloseParen);

                expression
            }

            MinusSign => {
                self.consume(MinusSign);
                let factor = self.get_factor();

                Box::new(Node::UnaryOp {
                    op_type: UnaryOpType::Negate,
                    operand: factor,
                })
            }

            _ => {
                panic!("Expected (+/-) number or opening parentheses.")
            }
        }
    }

    fn get_power(&mut self) -> Box<Node> {
        let mut factor = self.get_factor();

        while self.token().token_type == PowerSign {
            self.consume(PowerSign);
            factor = Box::new(Node::BinaryOp {
                op_type: BinaryOpType::Power,
                lhs: factor,
                rhs: self.get_factor(),
            });
        }

        factor
    }

    fn get_term(&mut self) -> Box<Node> {
        let mut power = self.get_power();

        while [TimesSign, DivideSign].contains(&self.token().token_type) {
            match self.token().token_type {
                TimesSign => {
                    self.consume(TimesSign);
                    power = Box::new(Node::BinaryOp {
                        op_type: BinaryOpType::Multiply,
                        lhs: power,
                        rhs: self.get_power(),
                    });
                }

                DivideSign => {
                    self.consume(DivideSign);
                    power = Box::new(Node::BinaryOp {
                        op_type: BinaryOpType::Divide,
                        lhs: power,
                        rhs: self.get_power(),
                    });
                }

                _ => {
                    panic!()
                }
            }
        }

        power
    }

    pub fn get_expression(&mut self) -> Box<Node> {
        let mut term = self.get_term();

        while [PlusSign, MinusSign].contains(&self.token().token_type) {
            match self.token().token_type {
                PlusSign => {
                    self.consume(PlusSign);
                    term = Box::new(Node::BinaryOp {
                        op_type: BinaryOpType::Add,
                        lhs: term,
                        rhs: self.get_term(),
                    })
                }

                MinusSign => {
                    self.consume(MinusSign);
                    term = Box::new(Node::BinaryOp {
                        op_type: BinaryOpType::Subtract,
                        lhs: term,
                        rhs: self.get_term(),
                    })
                }

                _ => {
                    panic!()
                }
            }
        }

        term
    }
}
