pub mod lexer;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8,
}
