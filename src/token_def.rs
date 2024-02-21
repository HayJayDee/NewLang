use crate::{lexable_token, token::{LexableToken, TokenType}};


pub const REGISTERED_TOKENS: &[LexableToken] = &[
    lexable_token!("+", TokenType::Plus),
    lexable_token!("-", TokenType::Minus),
    lexable_token!("{", TokenType::LeftBrace),
    lexable_token!("}", TokenType::RightBrace),
    lexable_token!("(", TokenType::LeftBracket),
    lexable_token!(")", TokenType::RightBracket),
    lexable_token!("=", TokenType::Equal),
];
