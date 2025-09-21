use std::collections::HashMap;
use std::sync::LazyLock;

// Macro to give `Debug` trait to TokenType
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    ILLEGAL, // For illegal characters
    EOF,     // Marks the end of file

    IDENT, // Strings like variable names
    INT,   // Numbers like 1 2 3...

    ASSIGN,       // =
    PLUS,         // +
    MINUS,        // -
    BANG,         // !
    ASTERISK,     // *
    FORWARDSLASH, // /

    GT,  // >
    LT,  // <
    GTE, // >=
    LTE, // <=

    EQ,  // ==
    NEQ, // !=

    COMMA,     // ,
    SEMICOLON, // ;

    LPAREN, // (
    RPAREN, // )
    LBRACE, // {
    RBRACE, // }

    FUNCTION, // fn
    LET,      // let
    IF,       // if
    ELSE,     // else
    TRUE,     // true
    FALSE,    // false
    RETURN,   // return
}

pub static KEYWORDS: LazyLock<HashMap<String, TokenType>> = LazyLock::new(|| {
    HashMap::from([
        (String::from("fn"), TokenType::FUNCTION),
        (String::from("let"), TokenType::LET),
        (String::from("if"), TokenType::IF),
        (String::from("else"), TokenType::ELSE),
        (String::from("true"), TokenType::TRUE),
        (String::from("false"), TokenType::FALSE),
        (String::from("return"), TokenType::RETURN),
    ])
});

pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}
