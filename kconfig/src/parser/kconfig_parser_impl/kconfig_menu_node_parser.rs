use crate::errors::parser_error::ParserError;
use crate::parser::kconfig_parser_impl::parser_traits::{Parseable, ParsingContext};
use crate::structure::nodes::KconfigMenuNode;

impl Parseable for KconfigMenuNode {
    fn parse(_context: &ParsingContext) -> Result<Self, ParserError> {
        Err(ParserError::Syntax("unimplemented".to_string()))
    }
}
