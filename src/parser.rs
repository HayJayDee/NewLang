use crate::{ast::AstNode, token::Token, token_def::TokenType};

const Operators: [(TokenType, u32); 2] = [
    (TokenType::Plus, 10),
    (TokenType::Minus, 10),
];

pub struct Parser;

impl Parser {

    fn parse_expr(position: &mut usize, tokens: &Vec<Token>) -> AstNode {
        match tokens[*position].token_type {
            TokenType::Number(number) => {
                *position += 2; // Skip ; for now
                AstNode::Int(number)
            },
            _ => {
                panic!();
            }
        }
    }

    fn parse_stmt(position: &mut usize, tokens: &Vec<Token>) -> AstNode {
        match &tokens[*position].token_type {
            TokenType::Identifier(name) => {
                if tokens[*position + 1].token_type == TokenType::Equal {
                    *position += 2;
                    let expr = Parser::parse_expr(position, tokens);
                    AstNode::VariableAssignment {
                        name: Box::new(name.clone()),
                        expr: Box::new(expr)
                    }
                }else {
                    panic!();
                }
            },
            _ => {
                panic!();
            }
        }
    }

    fn parse_block(position: &mut usize, tokens: &Vec<Token>) -> AstNode {
        match tokens[*position].token_type {
            TokenType::LeftBrace => {
                *position += 1;
                let mut stmts: Vec<AstNode> = Vec::new();
                while tokens[*position].token_type != TokenType::RightBrace {
                    stmts.push(Parser::parse_stmt(position, tokens));
                }
                // Skip Right Brace
                *position += 1;
                AstNode::Block {
                    stmts: Box::new(stmts)
                }
            },
            _ => {
                panic!();
            }
        }
    }

    fn parse_function_definition(position: &mut usize, tokens: &Vec<Token>) -> AstNode {

        let return_type = &tokens[*position];

        match &tokens[*position + 1].token_type {
            TokenType::Identifier(name) => {
                match tokens[*position + 2].token_type {
                    TokenType::LeftBracket => {
                        match tokens[*position + 3].token_type {
                            TokenType::RightBracket => {
                                match tokens[*position + 4].token_type {
                                    TokenType::LeftBrace => {
                                        *position += 4;
                                        let stmt = Parser::parse_block(position, tokens);
                                    
                                        return AstNode::FunctionDefinition {
                                            name: Box::new(name.clone()),
                                            return_type: Box::new(return_type.clone()),
                                            stmt: Box::new(stmt)
                                        };
                                    },
                                    _ => {
                                        panic!();
                                    }
                                }
                            },
                            _ => {
                                panic!();
                            }
                        }
                    },
                    _ => {
                        panic!();
                    }
                }
            },
            _ => {
                panic!("Error");
            }
        }
    }
    
    
    pub fn parse(tokens: Vec<Token>) -> Vec<AstNode> {
        let mut pos = 0;

        let mut nodes: Vec<AstNode> = Vec::new();

        while pos < tokens.len() {
            if tokens[pos] == TokenType::Keyword("void") {
                nodes.push(Parser::parse_function_definition(&mut pos, &tokens));
            }
        }
        
        nodes
    }
}


// match_types!(TokenType)
