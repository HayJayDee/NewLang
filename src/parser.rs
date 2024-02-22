use crate::{ast::AstNode, lexer::Lexer, token::Token, token_def::TokenType};

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let token = lexer.next().unwrap();
        Self {
            lexer,
            current_token: token,
        }
    }

    fn eat(&mut self, token_type: TokenType) {
        if self.current_token == token_type {
            self.current_token = self.lexer.next().unwrap();
        } else {
            panic!("Something went horribly wrong");
        }
    }

    fn consume(&mut self) {
        self.current_token = self.lexer.next().unwrap();
    }

    fn parse_factor(&mut self) -> AstNode {
        match self.current_token.token_type {
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
        }
    }

    fn parse_term(&mut self) -> AstNode {
        let mut node = self.parse_factor();

        while (self.current_token == TokenType::Multiply)
            || (self.current_token == TokenType::Divide)
        {
            let token = self.current_token.clone();
            self.consume();
            node = AstNode::BinaryOperation {
                op: Box::new(token),
                left: Box::new(node),
                right: Box::new(self.parse_factor()),
            };
        }
        node
    }

    fn parse_expression(&mut self) -> AstNode {
        let mut node = self.parse_term();
        while (self.current_token == TokenType::Minus) || (self.current_token == TokenType::Plus) {
            let token = self.current_token.clone();
            self.consume();
            node = AstNode::BinaryOperation {
                op: Box::new(token),
                left: Box::new(node),
                right: Box::new(self.parse_term()),
            };
        }
        node
    }

    pub fn parse(&mut self) -> AstNode {
        self.parse_expression()
    }
}
