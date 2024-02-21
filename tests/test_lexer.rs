use language::{
    self,
    lexer::Lexer,
    token::{Token, TokenType, REGISTERED_TOKENS},
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

#[test]
fn lexer_test_every_registered_token() {
    for token in REGISTERED_TOKENS {
        let input = token.match_str;
        let lexer = Lexer::new(input.to_string());
        let tokens = lexer.lex();
        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens[0],
            Token {
                content: token.match_str.to_string(),
                pos: 0,
                line: 1,
                token_type: token.token_type
            }
        );
    }
}

#[test]
fn lexer_test_identifier() {
    let input = "test _test tes_te _te_te_ _te123123_";
    let lexer = Lexer::new(input.to_string());
    let tokens = lexer.lex();
    test_token_vector(
        tokens,
        vec![
            Token {
                content: "test".to_string(),
                pos: 0,
                line: 1,
                token_type: TokenType::Identifier,
            },
            Token {
                content: "_test".to_string(),
                pos: 5,
                line: 1,
                token_type: TokenType::Identifier,
            },
            Token {
                content: "tes_te".to_string(),
                pos: 11,
                line: 1,
                token_type: TokenType::Identifier,
            },
            Token {
                content: "_te_te_".to_string(),
                pos: 18,
                line: 1,
                token_type: TokenType::Identifier,
            },
            Token {
                content: "_te123123_".to_string(),
                pos: 26,
                line: 1,
                token_type: TokenType::Identifier,
            },
        ],
    );
}
