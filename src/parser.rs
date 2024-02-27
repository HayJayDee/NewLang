use crate::{
    ast::AstNode,
    lexer::{Lexer, LexerError},
    token::Token,
    token_def::TokenType,
};

pub struct Parser {
    lexer: Lexer,
    current_token: Option<Token>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Result<Self, LexerError> {
        let token = lexer.lex_next()?;
        Ok(Self {
            lexer,
            current_token: token,
        })
    }

    fn eat(&mut self, token_type: TokenType) -> Result<(), LexerError> {
        match &self.current_token {
            Some(token) => {
                if *token == token_type {
                    let curr_token = self.lexer.lex_next()?;
                    self.current_token = curr_token;
                } else {
                    panic!("Something went horribly wrong");
                }
            }
            None => {
                panic!("Already reached EOF!");
            }
        }
        Ok(())
    }

    fn consume(&mut self) -> Result<(), LexerError> {
        self.current_token = self.lexer.lex_next()?;
        Ok(())
    }

    fn parse_factor(&mut self) -> Result<AstNode, LexerError> {
        match &self.current_token {
            Some(token) => match token.token_type {
                TokenType::Number(number) => {
                    self.consume()?;
                    Ok(AstNode::Int(number))
                }
                TokenType::LeftBracket => {
                    self.eat(TokenType::LeftBracket)?;
                    let node = self.parse_expression()?;
                    self.eat(TokenType::RightBracket)?;
                    Ok(node)
                }
                _ => {
                    // TODO: Return propper error
                    panic!("Unknown token {:?}", self.current_token);
                }
            },
            None => {
                // TODO: Return propper error
                panic!("Already reached EOF!");
            }
        }
    }

    fn parse_term(&mut self) -> Result<AstNode, LexerError> {
        let mut node = self.parse_factor()?;

        while let Some(token) = self.current_token.clone() {
            if (token == TokenType::Multiply) || (token == TokenType::Divide) {
                self.consume()?;
                node = AstNode::BinaryOperation {
                    op: Box::new(token),
                    left: Box::new(node),
                    right: Box::new(self.parse_factor()?),
                };
            } else {
                break;
            }
        }
        Ok(node)
    }

    fn parse_expression(&mut self) -> Result<AstNode, LexerError> {
        let mut node = self.parse_term()?;

        while let Some(token) = self.current_token.clone() {
            if (token == TokenType::Minus) || (token == TokenType::Plus) {
                self.consume()?;
                node = AstNode::BinaryOperation {
                    op: Box::new(token),
                    left: Box::new(node),
                    right: Box::new(self.parse_term()?),
                };
            } else {
                break;
            }
        }
        Ok(node)
    }

    pub fn parse(&mut self) -> Result<AstNode, LexerError> {
        self.parse_expression()
    }
}
