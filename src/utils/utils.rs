use crate::token::token::{KEYWORDS, TokenType};

pub fn is_letter(ch: u8) -> bool {
    return (b'a' <= ch && b'z' >= ch) || b'A' <= ch && b'Z' >= ch;
}

pub fn is_number(ch: u8) -> bool {
    return b'0' <= ch && ch <= b'9';
}

pub fn lookup_keyword(s: &String) -> TokenType {
    KEYWORDS.get(s).cloned().unwrap_or(TokenType::IDENT)
}
