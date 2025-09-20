use myn::lexer::*;
use myn::token::*;

#[test]
fn test_lexer_inputs() {
    let input: String = String::from("=+(){},;");
    let tests = [
        token::Token {
            token_type: token::TokenType::ASSIGN,
            literal: String::from("="),
        },
        token::Token {
            token_type: token::TokenType::PLUS,
            literal: String::from("+"),
        },
        token::Token {
            token_type: token::TokenType::LPAREN,
            literal: String::from("("),
        },
        token::Token {
            token_type: token::TokenType::RPAREN,
            literal: String::from(")"),
        },
        token::Token {
            token_type: token::TokenType::LBRACE,
            literal: String::from("{"),
        },
        token::Token {
            token_type: token::TokenType::RBRACE,
            literal: String::from("}"),
        },
        token::Token {
            token_type: token::TokenType::COMMA,
            literal: String::from(","),
        },
        token::Token {
            token_type: token::TokenType::SEMICOLON,
            literal: String::from(";"),
        },
        token::Token {
            token_type: token::TokenType::EOF,
            literal: String::from(""),
        },
    ];

    let mut l = lexer::new(input);

    for (i, expected) in tests.iter().enumerate() {
        let token = l.next_token();
        assert_eq!(
            token.token_type, expected.token_type,
            "tests[{}] - token type wrong",
            i
        );
        assert_eq!(
            token.literal, expected.literal,
            "tests[{}] - literal wrong",
            i
        );
    }
}
