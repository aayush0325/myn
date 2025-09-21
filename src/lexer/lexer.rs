use crate::token::token::{Token, TokenType};
use crate::utils::utils::{is_letter, is_number, lookup_keyword};

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

        // Follow a two pointer approach to parse
        // the current character and peek at the next character
        // note that both pointers start from 0 as defined in lexer::new(String x)
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> String {
        let start = self.position;

        while is_letter(self.ch) {
            self.read_char();
        }

        // slice from start to current position
        return self.input[start..self.position].to_string();
    }

    fn read_number(&mut self) -> String {
        let start = self.position;

        while is_number(self.ch) {
            self.read_char();
        }

        return self.input[start..self.position].to_string();
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            b'=' => Token {
                token_type: TokenType::ASSIGN,
                literal: String::from("="),
            },

            b';' => Token {
                token_type: TokenType::SEMICOLON,
                literal: String::from(";"),
            },

            b'(' => Token {
                token_type: TokenType::LPAREN,
                literal: String::from("("),
            },

            b')' => Token {
                token_type: TokenType::RPAREN,
                literal: String::from(")"),
            },

            b'{' => Token {
                token_type: TokenType::LBRACE,
                literal: String::from("{"),
            },

            b'}' => Token {
                token_type: TokenType::RBRACE,
                literal: String::from("}"),
            },

            b',' => Token {
                token_type: TokenType::COMMA,
                literal: String::from(","),
            },

            b'+' => Token {
                token_type: TokenType::PLUS,
                literal: String::from("+"),
            },

            0 => Token {
                token_type: TokenType::EOF,
                literal: String::from(""),
            },

            _ => {
                // Note that for numbers and strings we must return early
                // this is done to skip the read_char() call below as our
                // pointers are adjusted automatically via the util functions

                if is_letter(self.ch) {
                    let ident = self.read_identifier();
                    return Token {
                        // Borrow the string instead of cloning
                        token_type: lookup_keyword(&ident),
                        literal: ident,
                    };
                } else if is_number(self.ch) {
                    return Token {
                        token_type: TokenType::INT,
                        literal: self.read_number(),
                    };
                }
                Token {
                    token_type: TokenType::ILLEGAL,
                    literal: String::from(""),
                }
            }
        };

        // Increment the pointers after reading a token
        self.read_char();

        return tok;
    }

    fn skip_whitespace(&mut self) {
        while (self.ch == b' ') || (self.ch == b'\n') || (self.ch == b'\t') {
            self.read_char();
        }
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
    lexer.skip_whitespace();
    return lexer;
}
