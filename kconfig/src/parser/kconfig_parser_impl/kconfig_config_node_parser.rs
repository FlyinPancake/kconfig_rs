use crate::errors::parser_error::ParserError;
use crate::parser::kconfig_parser_impl::parser_traits::{Parseable, ParsingContext};
use crate::parser::utils::tokenizer::LineKConfigTokenizerIterator;
use crate::structure::nodes::KconfigConfigNode;

fn get_empty_config_node_from_header(
    context: &ParsingContext,
) -> Result<KconfigConfigNode, ParserError> {
    let header_line = context.span.get_source_span()[0];
    let mut header_tokens = LineKConfigTokenizerIterator::from_line(header_line);


}

impl Parseable for KconfigConfigNode {
    fn parse(context: &ParsingContext) -> Result<Self, ParserError> {
        let span = context.span;
        span.non_empty_or()?;
    }
}