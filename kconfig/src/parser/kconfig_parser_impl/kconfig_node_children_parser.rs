use crate::errors::parser_error::ParserError;
use crate::parser::constants::{END_MENU_KEYWORD, MENU_KEYWORD, COMMENT_KEYWORD, IF_KEYWORD, SOURCE_KEYWORD, ENDIF_KEYWORD, CONFIG_KEYWORD, MENU_CONFIG_KEYWORD, END_MENU_CONFIG_KEYWORD};
use crate::parser::kconfig_parser_impl::parser_traits::{Parseable, ParseableWithUnknownSpan, ParsingContext};
use crate::parser::kconfig_parser_impl::source_line_parser::parse_source_line;
use crate::parser::utils::find_index_of_next_end_keyword_in_context::find_index_of_next_end_keyword_in_context;
use crate::parser::utils::parse_span::ParseSpan;
use crate::parser::utils::tokenizer::LineKConfigTokenizerIterator;
use crate::structure::kconfig_node::KconfigNode;
use crate::structure::kconfig_node_children::KconfigNodeChildren;
use crate::structure::nodes::{KconfigConfigNode, KconfigIfNode, KconfigMenuConfigNode, KconfigMenuNode};

fn get_block_span<'a, 's, 'f>(
    context: &ParsingContext,
    keyword: &str,
    end_keyword: &str,
    end_span: &ParseSpan<'a, 's, 'f>,
) -> Option<ParseSpan<'a, 's, 'f>> {
    let end_at = find_index_of_next_end_keyword_in_context(keyword, end_keyword,
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

        let mut line_index = 0;
        while line_index < span.len() {
            let line = &span.get_source_span()[line_index];
            let mut tokens = LineKConfigTokenizerIterator::from_line(line);

            if let Some(token) = tokens.next() {
                match token {
                    MENU_KEYWORD => {
                        let menu_span = get_block_span(context, MENU_KEYWORD, END_MENU_KEYWORD, &span.get_with_start_at(line_index))
                            .ok_or(ParserError::syntax_in_span_at("Expected a menu end keyword after a menu keyword.", &span, line_index))?;
                        let menu = KconfigMenuNode::parse(&context.with_different_span(
                            &menu_span,
                        ))?;
                        line_index += menu_span.len();
                        node_children.add_children(KconfigNode::Menu(menu));
                        continue;
                    },
                    IF_KEYWORD => {
                        let if_span = get_block_span(context, IF_KEYWORD, ENDIF_KEYWORD, &span.get_with_start_at(line_index))
                            .ok_or(ParserError::syntax_in_span_at("Expected a if end keyword after an if keyword.", &span, line_index))?;
                        let if_node = KconfigIfNode::parse(&context.with_different_span(
                            &if_span,
                        ))?;
                        line_index += if_span.len();
                        node_children.add_children(KconfigNode::If(if_node));
                        continue;
                    },
                    COMMENT_KEYWORD => {},
                    SOURCE_KEYWORD => {
                        let children_from_sourcing = parse_source_line(&context.get_line_context_with_span(
                            &span.get_line_span_at(line_index),
                        ))?;
                        node_children.add_all_children(children_from_sourcing);
                    },
                    CONFIG_KEYWORD => {
                        let config_potential_span = span.get_with_start_at(line_index);
                        let (config, config_span) = KconfigConfigNode::parse_with_unknown_span(
                            &context.with_different_span(&config_potential_span),
                        )?;

                        line_index += config_span.len();
                        node_children.add_children(KconfigNode::Config(config));
                        continue;
                    },
                    MENU_CONFIG_KEYWORD => {
                        let menu_config_span = get_block_span(context, MENU_CONFIG_KEYWORD, END_MENU_CONFIG_KEYWORD, &span.get_with_start_at(line_index))
                            .ok_or(ParserError::syntax_in_span_at("Expected a menuconfig end keyword after a menuconfig keyword.", &span, line_index))?;
                        let menu_config = KconfigMenuConfigNode::parse(&context.with_different_span(
                            &menu_config_span,
                        ))?;
                        line_index += menu_config_span.len();
                        node_children.add_children(KconfigNode::MenuConfig(menu_config));
                        continue;
                    },
                    _ => {}
                }
            }

            line_index += 1;
        }

        Ok(node_children)
    }
}
