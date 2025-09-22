pub mod letstatement;
pub mod program;

pub trait Node {
    fn token_literal(&self) -> String;
}

pub trait Expression: Node {}

pub trait Statement: Node {}
