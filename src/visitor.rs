use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    values::{FunctionValue, IntValue},
};

use crate::{ast::AstNode, token_def::TokenType};

pub struct Visitor<'ctx> {
    context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    curr_func: Option<FunctionValue<'ctx>>,
}

impl<'ctx> Visitor<'ctx> {
    pub fn new(
        context: &'ctx Context,
        module: Module<'ctx>,
        builder: Builder<'ctx>,
    ) -> Visitor<'ctx> {
        Visitor {
            context,
            builder,
            module,
            curr_func: None,
        }
    }

    pub fn visit_number(&self, number: i64) -> IntValue {
        self.context.i64_type().const_int(number as u64, false)
    }

    pub fn visit_plus(&self, left: &AstNode, right: &AstNode) -> IntValue {
        let lhs = self.visit_expression(left);
        let rhs = self.visit_expression(right);
        self.builder.build_int_add(lhs, rhs, "").unwrap()
    }
    pub fn visit_minus(&self, left: &AstNode, right: &AstNode) -> IntValue {
        let lhs = self.visit_expression(left);
        let rhs = self.visit_expression(right);
        self.builder.build_int_sub(lhs, rhs, "").unwrap()
    }
    pub fn visit_multiply(&self, left: &AstNode, right: &AstNode) -> IntValue {
        let lhs = self.visit_expression(left);
        let rhs = self.visit_expression(right);
        self.builder.build_int_mul(lhs, rhs, "").unwrap()
    }
    pub fn visit_divide(&self, left: &AstNode, right: &AstNode) -> IntValue {
        let lhs = self.visit_expression(left);
        let rhs = self.visit_expression(right);
        self.builder.build_int_signed_div(lhs, rhs, "").unwrap()
    }

    fn visit_expression(&self, node: &AstNode) -> IntValue {
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

    pub fn visit(&mut self, node: &AstNode) {
        // Start with default setup for functions
        let i64_type = self.context.i64_type();
        let fn_type = i64_type.fn_type(&[], false);
        let function = self.module.add_function("sum", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");

        self.builder.position_at_end(basic_block);

        self.curr_func = Some(function);

        // Visit nodes
        let sum = self.visit_expression(node);

        self.builder.build_return(Some(&sum)).unwrap();
    }
}
