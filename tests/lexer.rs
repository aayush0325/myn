use myn::lexer::*;
use myn::token::*;

#[test]
fn test_lexer_inputs_simple() {
    let input: String = String::from("=+(){},;");
    let tests = [
        Token {
            token_type: TokenType::ASSIGN,
            literal: String::from("="),
        },
        Token {
            token_type: TokenType::PLUS,
            literal: String::from("+"),
        },
        Token {
            token_type: TokenType::LPAREN,
            literal: String::from("("),
        },
        Token {
            token_type: TokenType::RPAREN,
            literal: String::from(")"),
        },
        Token {
            token_type: TokenType::LBRACE,
            literal: String::from("{"),
        },
        Token {
            token_type: TokenType::RBRACE,
            literal: String::from("}"),
        },
        Token {
            token_type: TokenType::COMMA,
            literal: String::from(","),
        },
        Token {
            token_type: TokenType::SEMICOLON,
            literal: String::from(";"),
        },
        Token {
            token_type: TokenType::EOF,
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
fn test_lexer_inputs_keywords() {
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
        Token {
            token_type: TokenType::LET,
            literal: String::from("let"),
        },
        Token {
            token_type: TokenType::IDENT,
            literal: String::from("five"),
        },
        Token {
            token_type: TokenType::ASSIGN,
            literal: String::from("="),
        },
        Token {
            token_type: TokenType::INT,
            literal: String::from("5"),
        },
        Token {
            token_type: TokenType::SEMICOLON,
            literal: String::from(";"),
        },
        Token {
            token_type: TokenType::LET,
            literal: String::from("let"),
        },
        Token {
            token_type: TokenType::IDENT,
            literal: String::from("ten"),
        },
        Token {
            token_type: TokenType::ASSIGN,
            literal: String::from("="),
        },
        Token {
            token_type: TokenType::INT,
            literal: String::from("10"),
        },
        Token {
            token_type: TokenType::SEMICOLON,
            literal: String::from(";"),
        },
        Token {
            token_type: TokenType::LET,
            literal: String::from("let"),
        },
        Token {
            token_type: TokenType::IDENT,
            literal: String::from("add"),
        },
        Token {
            token_type: TokenType::ASSIGN,
            literal: String::from("="),
        },
        Token {
            token_type: TokenType::FUNCTION,
            literal: String::from("fn"),
        },
        Token {
            token_type: TokenType::LPAREN,
            literal: String::from("("),
        },
        Token {
            token_type: TokenType::IDENT,
            literal: String::from("x"),
        },
        Token {
            token_type: TokenType::COMMA,
            literal: String::from(","),
        },
        Token {
            token_type: TokenType::IDENT,
            literal: String::from("y"),
        },
        Token {
            token_type: TokenType::RPAREN,
            literal: String::from(")"),
        },
        Token {
            token_type: TokenType::LBRACE,
            literal: String::from("{"),
        },
        Token {
            token_type: TokenType::IDENT,
            literal: String::from("x"),
        },
        Token {
            token_type: TokenType::PLUS,
            literal: String::from("+"),
        },
        Token {
            token_type: TokenType::IDENT,
            literal: String::from("y"),
        },
        Token {
            token_type: TokenType::SEMICOLON,
            literal: String::from(";"),
        },
        Token {
            token_type: TokenType::RBRACE,
            literal: String::from("}"),
        },
        Token {
            token_type: TokenType::SEMICOLON,
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

#[test]
fn test_lexer_complete() {
    let input = String::from(
        "
        let five = 5;
        let ten = 10;
        let add = fn(x, y) {
            x + y;
        };
        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;
        if (5 < 10) {
            return true;
        } else {
            return false;
        }
        10 == 10;
        10 != 9;
        ",
    );

    let tests = [
        Token {
            token_type: TokenType::LET,
            literal: String::from("let"),
        },
        Token {
            token_type: TokenType::IDENT,
            literal: String::from("five"),
        },
        Token {
            token_type: TokenType::ASSIGN,
            literal: String::from("="),
        },
        Token {
            token_type: TokenType::INT,
            literal: String::from("5"),
        },
        Token {
            token_type: TokenType::SEMICOLON,
            literal: String::from(";"),
        },
        Token {
            token_type: TokenType::LET,
            literal: String::from("let"),
        },
        Token {
            token_type: TokenType::IDENT,
            literal: String::from("ten"),
        },
        Token {
            token_type: TokenType::ASSIGN,
            literal: String::from("="),
        },
        Token {
            token_type: TokenType::INT,
            literal: String::from("10"),
        },
        Token {
            token_type: TokenType::SEMICOLON,
            literal: String::from(";"),
        },
        Token {
            token_type: TokenType::LET,
            literal: String::from("let"),
        },
        Token {
            token_type: TokenType::IDENT,
            literal: String::from("add"),
        },
        Token {
            token_type: TokenType::ASSIGN,
            literal: String::from("="),
        },
        Token {
            token_type: TokenType::FUNCTION,
            literal: String::from("fn"),
        },
        Token {
            token_type: TokenType::LPAREN,
            literal: String::from("("),
        },
        Token {
            token_type: TokenType::IDENT,
            literal: String::from("x"),
        },
        Token {
            token_type: TokenType::COMMA,
            literal: String::from(","),
        },
        Token {
            token_type: TokenType::IDENT,
            literal: String::from("y"),
        },
        Token {
            token_type: TokenType::RPAREN,
            literal: String::from(")"),
        },
        Token {
            token_type: TokenType::LBRACE,
            literal: String::from("{"),
        },
        Token {
            token_type: TokenType::IDENT,
            literal: String::from("x"),
        },
        Token {
            token_type: TokenType::PLUS,
            literal: String::from("+"),
        },
        Token {
            token_type: TokenType::IDENT,
            literal: String::from("y"),
        },
        Token {
            token_type: TokenType::SEMICOLON,
            literal: String::from(";"),
        },
        Token {
            token_type: TokenType::RBRACE,
            literal: String::from("}"),
        },
        Token {
            token_type: TokenType::SEMICOLON,
            literal: String::from(";"),
        },
        Token {
            token_type: TokenType::LET,
            literal: String::from("let"),
        },
        Token {
            token_type: TokenType::IDENT,
            literal: String::from("result"),
        },
        Token {
            token_type: TokenType::ASSIGN,
            literal: String::from("="),
        },
        Token {
            token_type: TokenType::IDENT,
            literal: String::from("add"),
        },
        Token {
            token_type: TokenType::LPAREN,
            literal: String::from("("),
        },
        Token {
            token_type: TokenType::IDENT,
            literal: String::from("five"),
        },
        Token {
            token_type: TokenType::COMMA,
            literal: String::from(","),
        },
        Token {
            token_type: TokenType::IDENT,
            literal: String::from("ten"),
        },
        Token {
            token_type: TokenType::RPAREN,
            literal: String::from(")"),
        },
        Token {
            token_type: TokenType::SEMICOLON,
            literal: String::from(";"),
        },
        Token {
            token_type: TokenType::BANG,
            literal: String::from("!"),
        },
        Token {
            token_type: TokenType::MINUS,
            literal: String::from("-"),
        },
        Token {
            token_type: TokenType::FORWARDSLASH,
            literal: String::from("/"),
        },
        Token {
            token_type: TokenType::ASTERISK,
            literal: String::from("*"),
        },
        Token {
            token_type: TokenType::INT,
            literal: String::from("5"),
        },
        Token {
            token_type: TokenType::SEMICOLON,
            literal: String::from(";"),
        },
        Token {
            token_type: TokenType::INT,
            literal: String::from("5"),
        },
        Token {
            token_type: TokenType::LT,
            literal: String::from("<"),
        },
        Token {
            token_type: TokenType::INT,
            literal: String::from("10"),
        },
        Token {
            token_type: TokenType::GT,
            literal: String::from(">"),
        },
        Token {
            token_type: TokenType::INT,
            literal: String::from("5"),
        },
        Token {
            token_type: TokenType::SEMICOLON,
            literal: String::from(";"),
        },
        Token {
            token_type: TokenType::IF,
            literal: String::from("if"),
        },
        Token {
            token_type: TokenType::LPAREN,
            literal: String::from("("),
        },
        Token {
            token_type: TokenType::INT,
            literal: String::from("5"),
        },
        Token {
            token_type: TokenType::LT,
            literal: String::from("<"),
        },
        Token {
            token_type: TokenType::INT,
            literal: String::from("10"),
        },
        Token {
            token_type: TokenType::RPAREN,
            literal: String::from(")"),
        },
        Token {
            token_type: TokenType::LBRACE,
            literal: String::from("{"),
        },
        Token {
            token_type: TokenType::RETURN,
            literal: String::from("return"),
        },
        Token {
            token_type: TokenType::TRUE,
            literal: String::from("true"),
        },
        Token {
            token_type: TokenType::SEMICOLON,
            literal: String::from(";"),
        },
        Token {
            token_type: TokenType::RBRACE,
            literal: String::from("}"),
        },
        Token {
            token_type: TokenType::ELSE,
            literal: String::from("else"),
        },
        Token {
            token_type: TokenType::LBRACE,
            literal: String::from("{"),
        },
        Token {
            token_type: TokenType::RETURN,
            literal: String::from("return"),
        },
        Token {
            token_type: TokenType::FALSE,
            literal: String::from("false"),
        },
        Token {
            token_type: TokenType::SEMICOLON,
            literal: String::from(";"),
        },
        Token {
            token_type: TokenType::RBRACE,
            literal: String::from("}"),
        },
        Token {
            token_type: TokenType::INT,
            literal: String::from("10"),
        },
        Token {
            token_type: TokenType::EQ,
            literal: String::from("=="),
        },
        Token {
            token_type: TokenType::INT,
            literal: String::from("10"),
        },
        Token {
            token_type: TokenType::SEMICOLON,
            literal: String::from(";"),
        },
        Token {
            token_type: TokenType::INT,
            literal: String::from("10"),
        },
        Token {
            token_type: TokenType::NEQ,
            literal: String::from("!="),
        },
        Token {
            token_type: TokenType::INT,
            literal: String::from("9"),
        },
        Token {
            token_type: TokenType::SEMICOLON,
            literal: String::from(";"),
        },
        Token {
            token_type: TokenType::EOF,
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
