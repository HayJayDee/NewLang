use crate::token::Token;

#[derive(Debug)]
pub enum AstNode {
    Int(i64),
    BinaryOperation {
        op: Box<Token>,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    /*FunctionDefinition {
        name: Box<String>,
        return_type: Box<Token>,
        stmt: Box<AstNode>,
    },
    VariableAssignment {
        name: Box<String>,
        expr: Box<AstNode>,
    },
    Block {
        stmts: Box<Vec<AstNode>>,
    },*/
}
