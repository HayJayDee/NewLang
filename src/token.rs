
#[derive(Debug)]
pub struct Token {
    pub content: String,
    pub pos: usize,
    pub line: usize,
    pub token_type: TokenType,
}

#[derive(Debug)]
pub enum TokenType {
    Plus,
    Parenthesis,
    Braket,
    Identifier,
    Number,
    Equals
}

/*pub trait Token<T> {
    fn from_str(&self, text: String, pos: i32, line: i32) -> Box<dyn Token<T>>;
}*/

/*
impl Token {
    pub fn new(line: i32, pos: i32, content: String) -> Self {
        Self {
            line: line,
            pos: pos,
            content: content
        }
    }
}
*/