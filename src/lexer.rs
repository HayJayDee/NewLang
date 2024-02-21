use std::fmt::{self, Display, Formatter};

use crate::token::{
    Token,
    TokenType
};


#[derive(Clone, Debug)]
struct LexerError {
    line: usize,
    pos: usize,
    content: String
}

impl Display for LexerError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Unknown token at line {} position {}: '{}'", self.line, self.pos, self.content)
    }
}

pub struct Lexer {
    text: String,
}


impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            text: input
        }
    }

    fn lex_line(&self, line_num: usize, line: &str) -> Result<Vec<Token>, LexerError> {
        let mut pos = 1;

        let mut tokens: Vec<Token> = Vec::new();

        let mut chars = line.chars().peekable();

        while let Some(c) = chars.next() {

            if c.is_whitespace() {
                pos += 1;
                continue;
            }

            if c == '+' {
                tokens.push(
                    Token {
                        content: "+".to_string(),
                        pos: pos,
                        line: line_num,
                        token_type: TokenType::Plus
                    }
                );
                pos+=1;
                continue;
            }else if c == '(' {
                tokens.push(
                    Token {
                        content: "(".to_string(),
                        pos: pos,
                        line: line_num,
                        token_type: TokenType::Parenthesis
                    }
                );
                pos+=1;
                continue;
            }else if c == '=' {
                tokens.push(
                    Token {
                        content: "=".to_string(),
                        pos: pos,
                        line: line_num,
                        token_type: TokenType::Equals
                    }
                );
                pos+=1;
                continue;
            }else if c == ')' {
                tokens.push(
                    Token {
                        content: ")".to_string(),
                        pos: pos,
                        line: line_num,
                        token_type: TokenType::Parenthesis
                    }
                );
                pos+=1;
                continue;
            }else if c == '{' {
                tokens.push(
                    Token {
                        content: "{".to_string(),
                        pos: pos,
                        line: line_num,
                        token_type: TokenType::Braket
                    }
                );
                pos+=1;
                continue;
            }else if c == '}' {
                tokens.push(
                    Token {
                        content: "}".to_string(),
                        pos: pos,
                        line: line_num,
                        token_type: TokenType::Braket
                    }
                );
                pos+=1;
                continue;
            }else if c.is_alphabetic() || c == '_' {
                // Lex Identifier
                let mut identifier = c.to_string();
                let start_pos = pos;
                while let Some(identifier_char) = chars.peek() {
                    if identifier_char.is_alphanumeric() || identifier_char.to_owned() == '_' {
                        identifier.push(chars.next().unwrap());
                        pos += 1;
                    }else {
                        break;
                    }
                }
                tokens.push(
                    Token {
                        content: identifier,
                        pos: start_pos,
                        line: line_num,
                        token_type: TokenType::Identifier
                    }
                );
            }else if c.is_numeric() {
                // Lex Number
                let mut number = c.to_string();
                let start_pos = pos;
                while let Some(num_char) = chars.peek() {
                    if num_char.is_numeric() {
                        number.push(chars.next().unwrap());
                        pos += 1;
                    }else {
                        break;
                    }
                }
                tokens.push(
                    Token {
                        content: number,
                        pos: start_pos,
                        line: line_num,
                        token_type: TokenType::Number
                    }
                );
            }else {
                Err(LexerError {
                    pos: pos,
                    line: line_num,
                    content: c.to_string()
                })?
            }
        }

        Ok(tokens)
    }

    pub fn lex(&self) -> Vec<Token> {

        let lines = self.text.split("\n");

        let mut tokens: Vec<Token> = Vec::new();

        for (line_num, line) in lines.enumerate() {
            let line_tokens = self.lex_line(line_num, line).unwrap_or_else(|err| {
                println!("Lexer Error: {}", err);
                Vec::new()
            });


            tokens.extend(line_tokens);
        }

        tokens
    }
}
