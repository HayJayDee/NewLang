use language::{
    self,
    lexer::Lexer,
    token::{Token, TokenType},
};

fn test_token_vector(left: Vec<Token>, right: Vec<Token>) {
    for (index, left) in left.iter().enumerate() {
        assert_eq!(*left, right[index]);
    }
}

#[test]
fn lexer_test_tokens() {
    let test_string = "void main ( ) )) {}     =";
    let lexer = Lexer::new(test_string.to_string());
    let tokens = lexer.lex();

    test_token_vector(
        tokens,
        vec![
            Token {
                content: "void".to_string(),
                pos: 0,
                line: 1,
                token_type: TokenType::Identifier,
            },
            Token {
                content: "main".to_string(),
                pos: 5,
                line: 1,
                token_type: TokenType::Identifier,
            },
            Token {
                content: "(".to_string(),
                pos: 10,
                line: 1,
                token_type: TokenType::LeftBracket,
            },
            Token {
                content: ")".to_string(),
                pos: 12,
                line: 1,
                token_type: TokenType::RightBracket,
            },
            Token {
                content: ")".to_string(),
                pos: 14,
                line: 1,
                token_type: TokenType::RightBracket,
            },
            Token {
                content: ")".to_string(),
                pos: 15,
                line: 1,
                token_type: TokenType::RightBracket,
            },
            Token {
                content: "{".to_string(),
                pos: 17,
                line: 1,
                token_type: TokenType::LeftBrace,
            },
            Token {
                content: "}".to_string(),
                pos: 18,
                line: 1,
                token_type: TokenType::RightBrace,
            },
            Token {
                content: "=".to_string(),
                pos: 24,
                line: 1,
                token_type: TokenType::Equal,
            },
        ],
    );
}
