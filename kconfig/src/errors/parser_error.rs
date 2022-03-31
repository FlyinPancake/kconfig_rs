use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("syntax error:`{0}`")]
    Syntax(String),

    #[error("internal parser error:`{0}`")]
    Internal(String),
}
