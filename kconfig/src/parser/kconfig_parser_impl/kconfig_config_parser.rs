use crate::errors::parser_error::ParserError;
use crate::parser::kconfig_parser_impl::parser_traits::{ParseableWithUnknownSpan, ParsingContext};
use crate::parser::utils::parse_span::ParseSpan;
use crate::structure::kconfig_config::KconfigConfig;

impl ParseableWithUnknownSpan for KconfigConfig {
    fn parse_with_unknown_span<'c, 'p, 'a, 's, 'f>(
        context: &ParsingContext<'c, 'p, 'a, 's, 'f>
    ) -> Result<(Self, ParseSpan<'a, 's, 'f>), ParserError> {
        todo!()
    }
}