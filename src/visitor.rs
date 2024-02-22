use crate::{ast::AstNode, token_def::TokenType};

#[derive(Default)]
pub struct Visitor;

impl Visitor {
    pub fn visit_number(&self, number: i64) -> i64 {
        number
    }

    pub fn visit_plus(&self, left: &AstNode, right: &AstNode) -> i64 {
        self.visit(left) + self.visit(right)
    }
    pub fn visit_minus(&self, left: &AstNode, right: &AstNode) -> i64 {
        self.visit(left) - self.visit(right)
    }
    pub fn visit_multiply(&self, left: &AstNode, right: &AstNode) -> i64 {
        self.visit(left) * self.visit(right)
    }
    pub fn visit_divide(&self, left: &AstNode, right: &AstNode) -> i64 {
        self.visit(left) / self.visit(right)
    }

    pub fn visit(&self, node: &AstNode) -> i64 {
        match node {
            AstNode::Int(number) => self.visit_number(*number),
            AstNode::BinaryOperation { op, left, right } => match op.token_type {
                TokenType::Plus => self.visit_plus(left, right),
                TokenType::Minus => self.visit_minus(left, right),
                TokenType::Multiply => self.visit_multiply(left, right),
                TokenType::Divide => self.visit_divide(left, right),
                _ => {
                    panic!("Token {:?} is not a valid binary operator", op);
                }
            },
        }
    }
}
