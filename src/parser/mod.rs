use crate::lexer::*;
use crate::token::*;

pub struct Parser {
    lexer: Lexer,

    // Note that we are using Option<Token> here because
    // the field will be empty initially hence "None" or
    // it can be Some(Token)
    cur_token: Option<Token>,
    peek_token: Option<Token>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        // Initialize the parser with the tokens as None
        let mut parser = Parser {
            lexer: lexer,
            cur_token: None,
            peek_token: None,
        };

        // Advance twice to set cur_token and peek_token
        parser.next_token();
        parser.next_token();

        return parser;
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.take();
        self.peek_token = Some(self.lexer.next_token());
    }
}
