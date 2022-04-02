use thiserror::Error;
use crate::parser::utils::parse_span::ParseSpan;

#[derive(Error, Debug, Clone)]
pub enum ParserError {
    #[error("syntax error:`{0}`")]
    Syntax(String),

    #[error("internal parser error:`{0}`")]
    Internal(String),
}

impl ParserError {
    pub fn syntax(msg: &str) -> Self {
        Self::Syntax(msg.to_string())
    }

    pub fn syntax_in_span_at(msg: &str, span: &ParseSpan, offset: usize) -> Self {
        Self::Syntax(
            format!(
                "{} in {}, line:{}",
                msg,
                span.get_filename(),
                span.get_global_span().0 + offset,
            ),
        )
    }
}