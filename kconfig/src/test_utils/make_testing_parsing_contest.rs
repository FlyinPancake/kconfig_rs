use crate::parser::kconfig_parser_impl::parser_traits::ParsingContext;
use crate::parser::parser_config::ParserConfig;
use crate::parser::utils::parse_span::ParseSpan;

pub fn make_testing_parsing_contest(
    src: &str,
    config: ParserConfig,
) -> ParsingContext {
    let source: &'static str = Box::leak(Box::new(src.to_string()));
    let lines_iter = source.lines().collect::<Vec<&str>>();
    let span = ParseSpan::from_source(&lines_iter[..], "test");
    ParsingContext {
        config: Box::leak(Box::new(config)),
        span: &span,
    }
}