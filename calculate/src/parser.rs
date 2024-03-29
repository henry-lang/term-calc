use crate::{
    identifiers::{Context, Function},
    tokenizer::Token::{self, *},
};

/*
GRAMMAR:
expr: term ((+ | -) term)*
term: power ((* | /) power)*
power: factor ((^) factor)*
factor: NumLiteral |
        OpenParen expr CloseParen |
        NameLiteral OpenParen expr CloseParen |
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

    FunctionCall {
        func: Function,
        arg: ChildNode,
    },
}

pub struct Parser<'a> {
    current: usize,
    tokens: Vec<Token>,
    identifiers: &'a Context,
}

type NodeResult = Result<Box<Node>, String>;

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>, identifiers: &'a Context) -> Self {
        Self {
            current: 0,
            tokens,
            identifiers,
        }
    }

    fn token(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current + 1)
    }

    fn consume(&mut self, token: Token) -> Result<(), String> {
        if match (&token, self.token()) {
            (NumLiteral(_), NumLiteral(_)) => true,
            (NameLiteral(_), NameLiteral(_)) => true,
            _ => &token == self.token(),
        } {
            if self.current + 1 < self.tokens.len() {
                self.current += 1
            }
        } else {
            return Err(format!("Expected {:?}, found {:?}.", token, self.token()));
        }
        Ok(())
    }

    fn get_factor(&mut self) -> NodeResult {
        // TODO: Remove .clone() because it's inefficient - probably a micro optimization though.
        match self.token().clone() {
            NumLiteral(value) => {
                self.consume(NumLiteral(0.0))?;

                Ok(Box::new(Node::Number(value)))
            }

            NameLiteral(value) => match self.peek() {
                Some(OpenParen) => {
                    self.consume(NameLiteral(String::new()))?;
                    self.consume(OpenParen)?;
                    let func = *self
                        .identifiers
                        .get_func(&value)
                        .ok_or_else(|| format!("Function \"{}\" does not exist.", &value))?;
                    let arg = self.get_expression()?;
                    self.consume(CloseParen)?;
                    Ok(Box::new(Node::FunctionCall { func, arg }))
                }

                _ => {
                    self.consume(NameLiteral(String::new()))?;
                    let constant = *self
                        .identifiers
                        .get_constant(&value)
                        .ok_or_else(|| format!("Constant \"{}\" does not exist.", &value))?;
                    Ok(Box::new(Node::Number(constant)))
                }
            },

            OpenParen => {
                self.consume(OpenParen)?;
                let expression = self.get_expression()?;
                self.consume(CloseParen)?;

                Ok(expression)
            }

            MinusSign => {
                self.consume(MinusSign)?;
                let factor = self.get_factor()?;

                Ok(Box::new(Node::UnaryOp {
                    op_type: UnaryOpType::Negate,
                    operand: factor,
                }))
            }

            _ => Err("Expected expression.".into()),
        }
    }

    fn get_power(&mut self) -> NodeResult {
        let mut factor = self.get_factor()?;

        while *self.token() == PowerSign {
            self.consume(PowerSign)?;
            factor = Box::new(Node::BinaryOp {
                op_type: BinaryOpType::Power,
                lhs: factor,
                rhs: self.get_factor()?,
            });
        }

        Ok(factor)
    }

    fn get_term(&mut self) -> NodeResult {
        let mut power = self.get_power()?;

        while match self.token() {
            TimesSign => {
                self.consume(TimesSign)?;
                power = Box::new(Node::BinaryOp {
                    op_type: BinaryOpType::Multiply,
                    lhs: power,
                    rhs: self.get_power()?,
                });
                true
            }

            DivideSign => {
                self.consume(DivideSign)?;
                power = Box::new(Node::BinaryOp {
                    op_type: BinaryOpType::Divide,
                    lhs: power,
                    rhs: self.get_power()?,
                });
                true
            }

            _ => false,
        } {}

        Ok(power)
    }

    pub fn get_expression(&mut self) -> NodeResult {
        let mut term = self.get_term()?;

        while match self.token() {
            PlusSign => {
                self.consume(PlusSign)?;
                term = Box::new(Node::BinaryOp {
                    op_type: BinaryOpType::Add,
                    lhs: term,
                    rhs: self.get_term()?,
                });
                true
            }

            MinusSign => {
                self.consume(MinusSign)?;
                term = Box::new(Node::BinaryOp {
                    op_type: BinaryOpType::Subtract,
                    lhs: term,
                    rhs: self.get_term()?,
                });
                true
            }

            _ => false,
        } {}

        Ok(term)
    }
}
