use crate::errors::parser_error::ParserError;
use crate::parser::utils::rdp::ParseSpan;
use crate::structure::kconfig_node_children::KconfigNodeChildren;

pub fn parse_kconfig_node_children(span: &ParseSpan) -> Result<KconfigNodeChildren, ParserError> {
    let mut node_children = KconfigNodeChildren::new_empty();

    for line_index in 0..span.source_span.len() {
        let line = &span.source_span[line_index];
        let mut tokens = line.split_whitespace()
            .map(|el| el.trim())
            .filter(|el| !el.is_empty());

        if let Some(keyword_token) = tokens.next() {
            match keyword_token {
                "menu" => {

                },
                _ => {}
            }
        }
    }

    Err(ParserError::Syntax("asd".to_string()))
}