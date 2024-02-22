use std::fmt::{self, Display, Formatter};

use crate::{
    token::*,
    token_def::{TokenType, REGISTERED_TOKENS},
};

/// The error which will get thrown if there is a problem during lexing
#[derive(Clone, Debug)]
pub struct LexerError {
    line: usize,
    pos: usize,
    content: String,
}

impl Display for LexerError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Unknown token at line {} position {}: '{}'",
            self.line, self.pos, self.content
        )
    }
}

pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
    line: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            chars: input.chars().collect(),
            pos: 0,
            line: 1,
        }
    }

    pub fn get_next_token(&mut self) -> Result<Token, LexerError> {
        if self.pos < self.chars.len() {
            // Skip all whitespaces

            while let Some(c) = self.chars.get(self.pos) {
                if !c.is_whitespace() {
                    break;
                }
                if *c == '\n' {
                    self.line += 1;
                }
                self.pos += 1;
            }

            if self.pos >= self.chars.len() {
                return Ok(Token {
                    pos: self.pos,
                    line: self.line,
                    token_type: TokenType::Eof,
                });
            }

            // Check for a constant token
            for registered_token in REGISTERED_TOKENS {
                // Check if the line is even long enough
                if self.chars.len() - self.pos < registered_token.match_str.len() {
                    continue;
                }

                // TODO: Change this step, so we don't have to allocate this every time
                if registered_token.match_str.chars().collect::<Vec<char>>()
                    == self.chars[self.pos..self.pos + registered_token.match_str.len()]
                {
                    let start_pos = self.pos;
                    self.pos += registered_token.match_str.len();
                    return Ok(Token {
                        pos: start_pos,
                        line: self.line,
                        token_type: registered_token.token_type.clone(),
                    });
                }
            }

            // Lex identificators
            if let Some(c) = self.chars.get(self.pos) {
                if c.is_alphabetic() || *c == '_' {
                    let start_pos = self.pos;
                    self.pos += 1;
                    while let Some(c) = self.chars.get(self.pos) {
                        if !c.is_alphanumeric() && *c != '_' {
                            break;
                        }
                        self.pos += 1;
                    }
                    return Ok(Token {
                        pos: start_pos,
                        line: self.line,
                        token_type: TokenType::Identifier(
                            self.chars[start_pos..self.pos]
                                .to_vec()
                                .iter()
                                .collect::<String>(),
                        ),
                    });
                }
            } else {
                return Ok(Token {
                    pos: self.pos,
                    line: self.line,
                    token_type: TokenType::Eof,
                });
            }

            // Lex numbers
            if let Some(c) = self.chars.get(self.pos) {
                if c.is_numeric() {
                    let start_pos = self.pos;
                    self.pos += 1;
                    while let Some(c) = self.chars.get(self.pos) {
                        if !c.is_numeric() {
                            break;
                        }
                        self.pos += 1;
                    }
                    // Parse the string into a number
                    let string: String = self.chars[start_pos..self.pos].to_vec().iter().collect();
                    match string.as_str().parse() {
                        Ok(number) => {
                            return Ok(Token {
                                pos: start_pos,
                                line: self.line,
                                token_type: TokenType::Number(number),
                            });
                        }
                        Err(_) => Err(LexerError {
                            pos: self.pos,
                            line: self.line,
                            content: self.chars[self.pos].to_string(),
                        })?,
                    }
                }
            } else {
                return Ok(Token {
                    pos: self.pos,
                    line: self.line,
                    token_type: TokenType::Eof,
                });
            }
        }
        Ok(Token {
            pos: self.pos,
            line: self.line,
            token_type: TokenType::Eof,
        })
    }
}

impl Iterator for Lexer {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        let next_token = self.get_next_token().unwrap();
        if next_token == TokenType::Eof {
            None
        } else {
            Some(next_token)
        }
    }
}
