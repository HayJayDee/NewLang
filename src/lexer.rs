use std::fmt::{self, Display, Formatter};

use crate::{token::*, token_def::REGISTERED_TOKENS};

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
    text: String,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self { text: input }
    }

    fn lex_line(&self, line_num: usize, line: &str) -> Result<Vec<Token>, LexerError> {
        let mut pos = 0;

        let mut tokens: Vec<Token> = Vec::new();

        let chars: Vec<char> = line.chars().collect();
        let chars_len = chars.len();

        while pos < line.len() {
            let mut successfull = false;

            if chars[pos].is_whitespace() {
                pos += 1;
                continue;
            }

            for registered_token in REGISTERED_TOKENS {
                if chars_len - pos < registered_token.match_str.len() {
                    continue;
                }

                // Lex constant expressions like +, -, *
                // TODO: Change this step
                if registered_token.match_str.chars().collect::<Vec<char>>()
                    == chars[pos..pos + registered_token.match_str.len()]
                {
                    tokens.push(Token {
                        content: registered_token.match_str.to_string(),
                        pos,
                        line: line_num,
                        token_type: registered_token.token_type,
                    });
                    pos += registered_token.match_str.len();
                    successfull = true;
                    break;
                }
            }

            if successfull {
                continue;
            }

            // Lex identificators
            if chars[pos].is_alphabetic() || chars[pos] == '_' {
                let start_pos = pos;
                pos += 1;
                while let Some(c) = chars.get(pos) {
                    if !c.is_alphanumeric() && *c != '_' {
                        break;
                    }
                    pos += 1;
                }
                tokens.push(Token {
                    content: chars[start_pos..pos].to_vec().iter().collect(),
                    pos: start_pos,
                    line: line_num,
                    token_type: TokenType::Identifier,
                });
                continue;
            }

            if !successfull {
                Err(LexerError {
                    pos,
                    line: line_num,
                    content: chars[pos].to_string(),
                })?
            }
        }
        Ok(tokens)
    }

    pub fn lex(&self) -> Result<Vec<Token>, LexerError> {
        let lines = self.text.split('\n');

        let mut tokens: Vec<Token> = Vec::new();

        for (line_num, line) in lines.enumerate() {
            let line_tokens = self.lex_line(line_num + 1, line)?;

            tokens.extend(line_tokens);
        }

        Ok(tokens)
    }
}
