use crate::errors::parser_error::ParserError;
use crate::parser::constants::CONFIG_KEYWORD;
use crate::parser::kconfig_parser_impl::parser_traits::{ParseableWithUnknownSpan, ParsingContext};
use crate::parser::utils::parse_node_header_and_get_name::parse_node_header_and_get_name;
use crate::parser::utils::parse_span::ParseSpan;
use crate::structure::atoms::KconfigSymbol;
use crate::structure::kconfig_config::KconfigConfig;
use crate::structure::nodes::KconfigConfigNode;

impl ParseableWithUnknownSpan for KconfigConfigNode {
    fn parse_with_unknown_span<'c, 'p, 'a, 's, 'f>(
        context: &ParsingContext<'c, 'p, 'a, 's, 'f>
    ) -> Result<(Self, ParseSpan<'a, 's, 'f>), ParserError> {
        let span = context.span;
        span.non_empty_or()?;

        let config_symbol = KconfigSymbol::new(parse_node_header_and_get_name(
            &context.span.get_line_span_at(0),
            CONFIG_KEYWORD,
        )?);
        let config_potential_span = context.span.get_with_start_at(1);
        let (config, config_actual_span) = KconfigConfig::parse_with_unknown_span(
            &context.with_different_span(&config_potential_span),
        )?;

        Ok((
            KconfigConfigNode {
                symbol: config_symbol,
                config,
            },
            span.get_with_end_at(config_actual_span.len()),
        ))
    }
}

#[cfg(test)]
mod test {
    use crate::parser::kconfig_parser_impl::parser_traits::ParseableWithUnknownSpan;
    use crate::structure::nodes::KconfigConfigNode;
    use crate::structure::property::ConfigType;
    use crate::test_utils::make_testing_parsing_context::make_testing_parsing_context;

    #[test]
    fn happy_path_parses_node() {
        let context = make_testing_parsing_context(
            "config ALMAFA_SAJTOS\n\
            \tdepends on SAJTOS && KIFLI\n\

            \thelp\n\
            \t\tdepends lol\n\
            \t\tkeke sajt\n\
            \tdepends on ALMA\n\
            \thex if KECSKE\n\
            ",
            Default::default(),
        );

        let (config, span) = KconfigConfigNode::parse_with_unknown_span(
            &context,
        ).unwrap();

        assert_eq!(config.symbol.name, "ALMAFA_SAJTOS");
        let types = config.config.type_property.unwrap();
        assert_eq!(types.config_type, ConfigType::Hex);
        assert_eq!(types.if_dep_on_type.unwrap().expression.source, "KECSKE");

        assert_ne!(config.config.help_property.unwrap().help_text.len(), 0);
    }

}