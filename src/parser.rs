use crate::{ast::AstNode, lexer::Lexer, token::Token, token_def::TokenType};

pub struct Parser {
    lexer: Lexer,
    current_token: Option<Token>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let token = lexer.next();
        Self {
            lexer,
            current_token: token,
        }
    }

    fn eat(&mut self, token_type: TokenType) {
        match &self.current_token {
            Some(token) => {
                if *token == token_type {
                    self.current_token = self.lexer.next();
                } else {
                    panic!("Something went horribly wrong");
                }
            }
            None => {
                panic!("Already reached EOF!");
            }
        }
    }

    fn consume(&mut self) {
        self.current_token = self.lexer.next();
    }

    fn parse_factor(&mut self) -> AstNode {
        match &self.current_token {
            Some(token) => match token.token_type {
                TokenType::Number(number) => {
                    self.consume();
                    AstNode::Int(number)
                }
                TokenType::LeftBracket => {
                    self.eat(TokenType::LeftBracket);
                    let node = self.parse_expression();
                    self.eat(TokenType::RightBracket);
                    node
                }
                _ => {
                    panic!("Unknown token {:?}", self.current_token);
                }
            },
            None => {
                panic!("Already reached EOF!");
            }
        }
    }

    fn parse_term(&mut self) -> AstNode {
        let mut node = self.parse_factor();

        while let Some(token) = self.current_token.clone() {
            if (token == TokenType::Multiply) || (token == TokenType::Divide) {
                self.consume();
                node = AstNode::BinaryOperation {
                    op: Box::new(token),
                    left: Box::new(node),
                    right: Box::new(self.parse_factor()),
                };
            } else {
                break;
            }
        }
        node
    }

    fn parse_expression(&mut self) -> AstNode {
        let mut node = self.parse_term();

        while let Some(token) = self.current_token.clone() {
            if (token == TokenType::Minus) || (token == TokenType::Plus) {
                self.consume();
                node = AstNode::BinaryOperation {
                    op: Box::new(token),
                    left: Box::new(node),
                    right: Box::new(self.parse_term()),
                };
            } else {
                break;
            }
        }
        node
    }

    pub fn parse(&mut self) -> AstNode {
        self.parse_expression()
    }
}
