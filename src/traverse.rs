use crate::parser::{Node, UnaryOpType, BinaryOpType};

use Node::*;

pub fn traverse(node: Box<Node>) -> f64 {
    return match *node {
        Number(value) => value,

        UnaryOp { op_type: UnaryOpType::Negate, operand } => -traverse(operand),
        
        BinaryOp { op_type, lhs, rhs } => {
            let lhs = traverse(lhs);
            let rhs = traverse(rhs);

            match op_type {
                BinaryOpType::Add => lhs + rhs,
                BinaryOpType::Subtract => lhs - rhs,
                BinaryOpType::Multiply => lhs * rhs,
                BinaryOpType::Divide => lhs / rhs,
            }
        }
    }
}
