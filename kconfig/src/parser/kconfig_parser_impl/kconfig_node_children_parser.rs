use crate::errors::parser_error::ParserError;
use crate::parser::constants::{ END_MENU_KEYWORD, MENU_KEYWORD };
use crate::parser::kconfig_parser_impl::kconfig_menu_node_parser::parse_kconfig_menu_node;
use crate::parser::utils::find_index_of_next_keyword_in_span::find_index_of_next_keyword_in_span;
use crate::parser::utils::parse_span::ParseSpan;
use crate::parser::utils::tokenizer::LineKConfigTokenizerIterator;
use crate::structure::kconfig_node::KconfigNode;
use crate::structure::kconfig_node_children::KconfigNodeChildren;

pub fn parse_kconfig_node_children(span: &ParseSpan) -> Result<KconfigNodeChildren, ParserError> {
    let mut node_children = KconfigNodeChildren::new_empty();

    for line_index in 0..span.len() {
        let line = &span.get_source_span()[line_index];
        let mut tokens = LineKConfigTokenizerIterator::from_line(line);

        if let Some(token) = tokens.next() {
            match token {
                MENU_KEYWORD => {
                    let menu_end_at = find_index_of_next_keyword_in_span(END_MENU_KEYWORD, span)
                        .ok_or(ParserError::syntax("Expected a menu end keyword after a menu keyword."))?;
                    let menu_span = span.get_with_bounds(line_index, menu_end_at);
                    let menu = parse_kconfig_menu_node(&menu_span)?;

                    node_children.add_children(KconfigNode::Menu(menu));
                }
                _ => {}
            }
        }
    }

    Err(ParserError::Syntax("asd".to_string()))
}
