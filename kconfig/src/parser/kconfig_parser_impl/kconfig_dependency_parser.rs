use crate::errors::parser_error::ParserError;
use crate::parser::constants::{DEPENDS_KEYWORD, ON_KEYWORD};
use crate::parser::kconfig_parser_impl::parser_traits::{LineParsingContext, ParseableFromLine};
use crate::parser::utils::tokenizer::LineKConfigTokenizerIterator;
use crate::structure::atoms::kconfig_dependency::KconfigDependency;
use crate::structure::atoms::kconfig_expression::KconfigExpression;

impl ParseableFromLine for KconfigDependency {
    fn parse_from_line(context: &LineParsingContext) -> Result<Self, ParserError> {
        let mut tokens = LineKConfigTokenizerIterator::from_line(context.line.get_line());

        if !tokens.next()
            .contains(DEPENDS_KEYWORD) {
            return Err(ParserError::syntax_in_line_span("Expected depends keyword here", &context.line))
        }

        if !tokens.next()
            .contains(ON_KEYWORD) {
            return Err(ParserError::syntax_in_line_span("Expected on keyword here", &context.line))
        }

        let expr_source = tokens.get_remaining_slice().trim();
    }
}