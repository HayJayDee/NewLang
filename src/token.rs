#[derive(Debug)]
pub struct Token {
    pub content: String,
    pub pos: usize,
    pub line: usize,
    pub token_type: TokenType,
}

impl PartialEq<&str> for Token {
    fn eq(&self, other: &&str) -> bool {
        self.content == *other.to_string()
    }
}

impl PartialEq<String> for Token {
    fn eq(&self, other: &String) -> bool {
        self.content == *other
    }
}

impl PartialEq<TokenType> for Token {
    fn eq(&self, other: &TokenType) -> bool {
        self.token_type == *other
    }
}

impl PartialEq<Token> for Token {
    fn eq(&self, other: &Token) -> bool {
        (*self == other.token_type)
            && (*self == other.content)
            && (self.line == other.line)
            && (self.pos == other.pos)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    Plus,
    Minus,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Identifier,
    Equal,
}

pub struct LexableToken {
    pub match_str: &'static str,
    pub token_type: TokenType,
}

#[macro_export]
macro_rules! lexable_token {
    ($match_str:expr, $token:expr) => {
        LexableToken {
            match_str: $match_str,
            token_type: $token,
        }
    };
}
