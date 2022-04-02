use crate::errors::parser_error::ParserError;
use crate::parser::constants::{END_MENU_KEYWORD, MENU_KEYWORD, COMMENT_KEYWORD, IF_KEYWORD, SOURCE_KEYWORD, ENDIF_KEYWORD};
use crate::parser::kconfig_parser_impl::parser_traits::{Parseable, ParsingContext};
use crate::parser::utils::find_index_of_next_keyword_in_context::find_index_of_next_keyword_in_context;
use crate::parser::utils::parse_span::ParseSpan;
use crate::parser::utils::tokenizer::LineKConfigTokenizerIterator;
use crate::structure::kconfig_node::KconfigNode;
use crate::structure::kconfig_node_children::KconfigNodeChildren;
use crate::structure::nodes::{KconfigIfNode, KconfigMenuNode};

fn get_block_span<'a, 's, 'f>(
    context: &ParsingContext,
    end_keyword: &str,
    end_span: &ParseSpan<'a, 's, 'f>,
) -> Option<ParseSpan<'a, 's, 'f>> {
    let end_at = find_index_of_next_keyword_in_context(end_keyword,
        &context.with_different_span(
            &end_span,
        ),
    )?;

    Some(end_span.get_with_end_at(end_at))
}

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
                        let menu_span = get_block_span(context, END_MENU_KEYWORD, &span.get_with_start_at(line_index))
                            .ok_or(ParserError::syntax("Expected a menu end keyword after a menu keyword."))?;
                        let menu = KconfigMenuNode::parse(&context.with_different_span(
                            &menu_span,
                        ))?;

                        node_children.add_children(KconfigNode::Menu(menu));
                    }
                    IF_KEYWORD => {
                        let if_span = get_block_span(context, ENDIF_KEYWORD, &span.get_with_start_at(line_index))
                            .ok_or(ParserError::syntax("Expected a if end keyword after a menu keyword."))?;
                        let if_node = KconfigIfNode::parse(&context.with_different_span(
                            &if_span,
                        ))?;

                        node_children.add_children(KconfigNode::If(if_node));
                    },
                    COMMENT_KEYWORD => {},
                    SOURCE_KEYWORD => {},
                    _ => {}
                }
            }
        }

        Ok(node_children)
    }
}
