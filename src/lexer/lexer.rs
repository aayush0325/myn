use crate::token::*;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> token::Token {
        let tok = match self.ch {
            b'=' => token::Token {
                token_type: token::TokenType::ASSIGN,
                literal: String::from("="),
            },

            b';' => token::Token {
                token_type: token::TokenType::SEMICOLON,
                literal: String::from(";"),
            },

            b'(' => token::Token {
                token_type: token::TokenType::LPAREN,
                literal: String::from("("),
            },

            b')' => token::Token {
                token_type: token::TokenType::RPAREN,
                literal: String::from(")"),
            },

            b'{' => token::Token {
                token_type: token::TokenType::LBRACE,
                literal: String::from("{"),
            },

            b'}' => token::Token {
                token_type: token::TokenType::RBRACE,
                literal: String::from("}"),
            },

            b',' => token::Token {
                token_type: token::TokenType::COMMA,
                literal: String::from(","),
            },

            b'+' => token::Token {
                token_type: token::TokenType::PLUS,
                literal: String::from("+"),
            },

            0 => token::Token {
                token_type: token::TokenType::EOF,
                literal: String::from(""),
            },

            _ => token::Token {
                token_type: token::TokenType::ILLEGAL,
                literal: String::from(""),
            },
        };

        self.read_char();

        return tok;
    }
}

pub fn new(input: String) -> Lexer {
    let mut lexer = Lexer {
        input: input,
        position: 0,
        read_position: 0,
        ch: 0,
    };
    lexer.read_char();
    return lexer;
}
