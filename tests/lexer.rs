use myn::lexer::*;
use myn::token::*;

#[test]
fn test_lexer_inputs_simple() {
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

#[test]
fn test_lexer_inputs_complete() {
    let input = String::from(
        "
        let five = 5;
        let ten = 10;
        let add = fn(x, y) {
            x + y;
        };
        let result = add(five, ten);
        ",
    );

    let tests = [
        token::Token {
            token_type: token::TokenType::LET,
            literal: String::from("let"),
        },
        token::Token {
            token_type: token::TokenType::IDENT,
            literal: String::from("five"),
        },
        token::Token {
            token_type: token::TokenType::ASSIGN,
            literal: String::from("="),
        },
        token::Token {
            token_type: token::TokenType::INT,
            literal: String::from("5"),
        },
        token::Token {
            token_type: token::TokenType::SEMICOLON,
            literal: String::from(";"),
        },
        token::Token {
            token_type: token::TokenType::LET,
            literal: String::from("let"),
        },
        token::Token {
            token_type: token::TokenType::IDENT,
            literal: String::from("ten"),
        },
        token::Token {
            token_type: token::TokenType::ASSIGN,
            literal: String::from("="),
        },
        token::Token {
            token_type: token::TokenType::INT,
            literal: String::from("10"),
        },
        token::Token {
            token_type: token::TokenType::SEMICOLON,
            literal: String::from(";"),
        },
        token::Token {
            token_type: token::TokenType::LET,
            literal: String::from("let"),
        },
        token::Token {
            token_type: token::TokenType::IDENT,
            literal: String::from("add"),
        },
        token::Token {
            token_type: token::TokenType::ASSIGN,
            literal: String::from("="),
        },
        token::Token {
            token_type: token::TokenType::FUNCTION,
            literal: String::from("fn"),
        },
        token::Token {
            token_type: token::TokenType::LPAREN,
            literal: String::from("("),
        },
        token::Token {
            token_type: token::TokenType::IDENT,
            literal: String::from("x"),
        },
        token::Token {
            token_type: token::TokenType::COMMA,
            literal: String::from(","),
        },
        token::Token {
            token_type: token::TokenType::IDENT,
            literal: String::from("y"),
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
            token_type: token::TokenType::IDENT,
            literal: String::from("x"),
        },
        token::Token {
            token_type: token::TokenType::PLUS,
            literal: String::from("+"),
        },
        token::Token {
            token_type: token::TokenType::IDENT,
            literal: String::from("y"),
        },
        token::Token {
            token_type: token::TokenType::SEMICOLON,
            literal: String::from(";"),
        },
        token::Token {
            token_type: token::TokenType::RBRACE,
            literal: String::from("}"),
        },
        token::Token {
            token_type: token::TokenType::SEMICOLON,
            literal: String::from(";"),
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
