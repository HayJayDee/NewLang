use crate::{constant_token, token::ConstantToken};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    /// Token +
    Plus,
    // Token ;
    Semicolon,
    /// Token -
    Minus,
    /// Token {
    LeftBrace,
    /// Token }
    RightBrace,
    /// Token (
    LeftBracket,
    /// Token )
    RightBracket,
    /// An Identifier token
    Identifier(String),
    /// Token =
    Equal,
    /// An integer token
    Number(usize),
    /// Token ++
    Increment,
}

/// An array of every constant token, so the lexer can access them easier
pub const REGISTERED_TOKENS: [ConstantToken; 9] = [
    constant_token!("++", TokenType::Increment),
    constant_token!("+", TokenType::Plus),
    constant_token!(";", TokenType::Semicolon),
    constant_token!("-", TokenType::Minus),
    constant_token!("{", TokenType::LeftBrace),
    constant_token!("}", TokenType::RightBrace),
    constant_token!("(", TokenType::LeftBracket),
    constant_token!(")", TokenType::RightBracket),
    constant_token!("=", TokenType::Equal),
];
