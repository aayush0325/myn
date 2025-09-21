use std::collections::HashMap;
use std::sync::LazyLock;

// Macro to give `Debug` trait to TokenType
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    IDENT,
    INT,

    ASSIGN,
    PLUS,

    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    FUNCTION,
    LET,
}

pub static KEYWORDS: LazyLock<HashMap<String, TokenType>> = LazyLock::new(|| {
    HashMap::from([
        (String::from("fn"), TokenType::FUNCTION),
        (String::from("let"), TokenType::LET),
    ])
});

pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}
