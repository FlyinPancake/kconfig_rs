use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("syntax error:`{0}`")]
    Syntax(String),
}