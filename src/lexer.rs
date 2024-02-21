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
    text: String,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self { text: input }
    }

    /// Lexing one line
    /// # Arguments
    /// * `line_num` - The line number of the current line
    /// * `line` - The actual line which will be lexed
    /// # Returns
    /// The lexed tokens or an error, it there was an unknown character
    fn lex_line(&self, line_num: usize, line: &str) -> Result<Vec<Token>, LexerError> {
        let mut pos = 0;
        let mut tokens: Vec<Token> = Vec::new();

        let chars: Vec<char> = line.chars().collect();
        let chars_len = chars.len();

        while pos < line.len() {
            let mut successfull = false;

            // Skip all whitespaces
            if chars[pos].is_whitespace() {
                pos += 1;
                continue;
            }

            // Check for a constant token
            for registered_token in REGISTERED_TOKENS {
                // Check if the line is even long enough
                if chars_len - pos < registered_token.match_str.len() {
                    continue;
                }

                // TODO: Change this step, so we don't have to allocate this every time
                if registered_token.match_str.chars().collect::<Vec<char>>()
                    == chars[pos..pos + registered_token.match_str.len()]
                {
                    tokens.push(Token {
                        pos,
                        line: line_num,
                        token_type: registered_token.token_type.clone(),
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
                    pos: start_pos,
                    line: line_num,
                    token_type: TokenType::Identifier(
                        chars[start_pos..pos].to_vec().iter().collect(),
                    ),
                });
                continue;
            }

            // Lex numbers
            if chars[pos].is_numeric() {
                let start_pos = pos;
                pos += 1;
                while let Some(c) = chars.get(pos) {
                    if !c.is_numeric() {
                        break;
                    }
                    pos += 1;
                }
                // Parse the string into a number
                let string: String = chars[start_pos..pos].to_vec().iter().collect();
                match string.as_str().parse() {
                    Ok(number) => {
                        tokens.push(Token {
                            pos: start_pos,
                            line: line_num,
                            token_type: TokenType::Number(number),
                        });
                    }
                    Err(_) => Err(LexerError {
                        pos,
                        line: line_num,
                        content: chars[pos].to_string(),
                    })?,
                }
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

    /// Lex the text of the Lexer
    /// # Returns
    /// A list of all parsed vectors or the error
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
