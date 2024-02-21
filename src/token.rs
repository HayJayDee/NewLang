use crate::token_def::TokenType;

/// The representation of a token
#[derive(Debug)]
pub struct Token {
    /// The position in the line (0 is first)
    pub pos: usize,
    /// The line number (1 is first)
    pub line: usize,
    /// The token type with additional information
    pub token_type: TokenType,
}

impl PartialEq<TokenType> for Token {
    fn eq(&self, other: &TokenType) -> bool {
        self.token_type == *other
    }
}

impl PartialEq<Token> for Token {
    fn eq(&self, other: &Token) -> bool {
        (*self == other.token_type) && (self.line == other.line) && (self.pos == other.pos)
    }
}

/// This is the representation of a constant token
pub struct ConstantToken {
    /// The string that the lexer will match against
    pub match_str: &'static str,
    pub token_type: TokenType,
}

/// A small macro to add a constant token into token_def.rs
#[macro_export]
macro_rules! constant_token {
    ($match_str:expr, $token:expr) => {
        ConstantToken {
            match_str: $match_str,
            token_type: $token,
        }
    };
}
