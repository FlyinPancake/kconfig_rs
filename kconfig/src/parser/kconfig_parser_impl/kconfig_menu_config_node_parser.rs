use crate::errors::parser_error::ParserError;
use crate::parser::constants::MENU_CONFIG_KEYWORD;
use crate::parser::kconfig_parser_impl::parser_traits::{Parseable, ParseableWithUnknownSpan, ParsingContext};
use crate::parser::utils::parse_node_header_and_get_name::parse_node_header_and_get_name;
use crate::structure::atoms::KconfigSymbol;
use crate::structure::kconfig_config::KconfigConfig;
use crate::structure::kconfig_node_children::KconfigNodeChildren;
use crate::structure::nodes::KconfigMenuConfigNode;

impl Parseable for KconfigMenuConfigNode {
    fn parse(context: &ParsingContext) -> Result<Self, ParserError> {
        let span = context.span;
        span.non_empty_or()?;

        let config_symbol = KconfigSymbol::new(parse_node_header_and_get_name(
            &context.span.get_line_span_at(0),
            MENU_CONFIG_KEYWORD,
        )?);
        let config_potential_span = context.span.get_with_start_at(1);
        let (config, config_actual_span) = KconfigConfig::parse_with_unknown_span(
            &context.with_different_span(&config_potential_span),
        )?;
        let children_span = span.get_with_start_at(config_actual_span.len());
        let children = KconfigNodeChildren::parse(
            &context.with_different_span(&children_span),
        )?;

        Ok(Self {
            symbol: config_symbol,
            children,
            config,
        })
    }
}