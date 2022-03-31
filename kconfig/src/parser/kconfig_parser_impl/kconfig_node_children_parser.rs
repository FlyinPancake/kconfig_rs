use crate::errors::parser_error::ParserError;
use crate::parser::utils::rdp::ParseSpan;
use crate::parser::utils::tokenizer::LineKConfigTokenizerIterator;
use crate::structure::kconfig_node_children::KconfigNodeChildren;

pub fn parse_kconfig_node_children(span: &ParseSpan) -> Result<KconfigNodeChildren, ParserError> {
    let mut node_children = KconfigNodeChildren::new_empty();

    for line_index in 0..span.source_span.len() {
        let line = &span.source_span[line_index];
        let mut tokens = LineKConfigTokenizerIterator::from_line(line);

        if let Some(token) = tokens.next() {
            match token {
                "menu" => {
                    //let menu_span = ParseSpan::new(span.source_span[line_index..])
                },
                _ => {}
            }
        }
    }

    Err(ParserError::Syntax("asd".to_string()))
}