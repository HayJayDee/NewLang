use photonix::{
    self,
    lexer::Lexer,
    token::Token,
    token_def::{TokenType, REGISTERED_TOKENS},
};

fn test_token_vector(left: Vec<Token>, right: Vec<Token>) {
    for (index, left) in left.iter().enumerate() {
        assert_eq!(*left, right[index]);
    }
}

#[test]
fn lexer_test_tokens() {
    let test_string = "void main ( ) )) {}     =;";
    let lexer = Lexer::new(test_string.to_string());
    let tokens = lexer.collect();

    test_token_vector(
        tokens,
        vec![
            Token {
                pos: 0,
                line: 1,
                token_type: TokenType::Keyword("void"),
            },
            Token {
                pos: 5,
                line: 1,
                token_type: TokenType::Identifier("main".to_string()),
            },
            Token {
                pos: 10,
                line: 1,
                token_type: TokenType::LeftBracket,
            },
            Token {
                pos: 12,
                line: 1,
                token_type: TokenType::RightBracket,
            },
            Token {
                pos: 14,
                line: 1,
                token_type: TokenType::RightBracket,
            },
            Token {
                pos: 15,
                line: 1,
                token_type: TokenType::RightBracket,
            },
            Token {
                pos: 17,
                line: 1,
                token_type: TokenType::LeftBrace,
            },
            Token {
                pos: 18,
                line: 1,
                token_type: TokenType::RightBrace,
            },
            Token {
                pos: 24,
                line: 1,
                token_type: TokenType::Equal,
            },
            Token {
                pos: 25,
                line: 1,
                token_type: TokenType::Semicolon,
            },
        ],
    );
}

#[test]
fn lexer_test_every_registered_token() {
    for token in REGISTERED_TOKENS {
        let input = token.match_str;
        let lexer = Lexer::new(input.to_string());
        let tokens: Vec<Token> = lexer.collect();
        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens[0],
            Token {
                pos: 0,
                line: 1,
                token_type: token.token_type.clone()
            }
        );
    }
}

#[test]
fn lexer_test_identifier() {
    let input = "test _test tes_te _te_te_ _te123123_";
    let lexer = Lexer::new(input.to_string());
    let tokens = lexer.collect();
    test_token_vector(
        tokens,
        vec![
            Token {
                pos: 0,
                line: 1,
                token_type: TokenType::Identifier("test".to_string()),
            },
            Token {
                pos: 5,
                line: 1,
                token_type: TokenType::Identifier("_test".to_string()),
            },
            Token {
                pos: 11,
                line: 1,
                token_type: TokenType::Identifier("tes_te".to_string()),
            },
            Token {
                pos: 18,
                line: 1,
                token_type: TokenType::Identifier("_te_te_".to_string()),
            },
            Token {
                pos: 26,
                line: 1,
                token_type: TokenType::Identifier("_te123123_".to_string()),
            },
        ],
    );
}

#[test]
pub fn test_lexer_utf8() {
    // TODO: Fix this test
    /*
    let input = "©";
    let lexer = Lexer::new(input.to_string());
    match lexer.next() {
        Ok(_) => {
            assert!(false)
        }
        Err(err) => {
            println!("{}", err);
        }
    }*/
}

#[test]
pub fn test_numbers() {
    let input = "12345 0345";
    let lexer = Lexer::new(input.to_string());
    let tokens = lexer.collect();
    test_token_vector(
        tokens,
        vec![
            Token {
                pos: 0,
                line: 1,
                token_type: TokenType::Number(12345),
            },
            Token {
                pos: 6,
                line: 1,
                token_type: TokenType::Number(345),
            },
        ],
    );
}
