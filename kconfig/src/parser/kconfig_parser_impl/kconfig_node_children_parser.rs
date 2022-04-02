use crate::errors::parser_error::ParserError;
use crate::parser::constants::{END_MENU_KEYWORD, MENU_KEYWORD};
use crate::parser::kconfig_parser_impl::parser_traits::{Parseable, ParsingContext};
use crate::parser::utils::find_index_of_next_keyword_in_context::find_index_of_next_keyword_in_context;
use crate::parser::utils::tokenizer::LineKConfigTokenizerIterator;
use crate::structure::kconfig_node::KconfigNode;
use crate::structure::kconfig_node_children::KconfigNodeChildren;
use crate::structure::nodes::KconfigMenuNode;

impl Parseable for KconfigNodeChildren {
    fn parse(context: &ParsingContext) -> Result<Self, ParserError> {
        let span = context.span;
        let mut node_children = KconfigNodeChildren::new_empty();

        for line_index in 0..span.len() {
            let line = &span.get_source_span()[line_index];
            let mut tokens = LineKConfigTokenizerIterator::from_line(line);

            if let Some(token) = tokens.next() {
                match token {
                    MENU_KEYWORD => {
                        let menu_end_span = span.get_with_start_at(line_index);
                        let menu_end_at = find_index_of_next_keyword_in_context(END_MENU_KEYWORD,
                                                                                &context.with_different_span(
                                                                                    &menu_end_span,
                                                                                ),
                        )
                            .ok_or(ParserError::syntax("Expected a menu end keyword after a menu keyword."))?;
                        let menu_span = span.get_with_bounds(line_index, menu_end_at);
                        let menu = KconfigMenuNode::parse(&context.with_different_span(
                            &menu_span,
                        ))?;

                        node_children.add_children(KconfigNode::Menu(menu));
                    }
                    _ => {}
                }
            }
        }

        Ok(node_children)
    }
}
