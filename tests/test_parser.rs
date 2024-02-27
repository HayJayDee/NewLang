use photonix::{ast::AstNode, lexer::Lexer, parser::Parser, token::Token, token_def::TokenType};

#[test]
pub fn test_parser_simple_bin_op() {
    let inputs = vec![
        ("1+2", TokenType::Plus),
        ("1-2", TokenType::Minus),
        ("1*2", TokenType::Multiply),
        ("1/2", TokenType::Divide),
    ];

    for input in inputs {
        let lexer = Lexer::new(input.0.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse().unwrap();

        assert_eq!(
            ast,
            AstNode::BinaryOperation {
                op: Box::new(Token {
                    line: 1,
                    pos: 1,
                    token_type: input.1
                }),
                left: Box::new(AstNode::Int(1)),
                right: Box::new(AstNode::Int(2))
            }
        );
    }
}

#[test]
pub fn test_parser_simple_parant() {
    let inputs = vec![
        ("(1+2)*2", TokenType::Plus),
        ("(1-2)*2", TokenType::Minus),
        ("(1*2)*2", TokenType::Multiply),
        ("(1/2)*2", TokenType::Divide),
    ];

    for input in inputs {
        let lexer = Lexer::new(input.0.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse().unwrap();

        assert_eq!(
            ast,
            AstNode::BinaryOperation {
                op: Box::new(Token {
                    line: 1,
                    pos: 5,
                    token_type: TokenType::Multiply
                }),
                left: Box::new(AstNode::BinaryOperation {
                    op: Box::new(Token {
                        pos: 2,
                        line: 1,
                        token_type: input.1
                    }),
                    left: Box::new(AstNode::Int(1)),
                    right: Box::new(AstNode::Int(2))
                }),
                right: Box::new(AstNode::Int(2))
            }
        );
    }
}

#[test]
#[should_panic]
pub fn test_parser_wrong_token() {
    let input = "1+ + 1";
    let lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer).unwrap();
    let _ = parser.parse();
}
