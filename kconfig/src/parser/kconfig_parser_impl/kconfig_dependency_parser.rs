use crate::errors::parser_error::ParserError;
use crate::parser::kconfig_parser_impl::parser_traits::{LineParsingContext, ParseableFromLine};
use crate::structure::atoms::kconfig_dependency::KconfigDependency;

impl ParseableFromLine for KconfigDependency {
    fn parse_from_line(_context: &LineParsingContext) -> Result<Self, ParserError> {
        Err(ParserError::syntax("unimpl"))
    }
}