use crate::errors::parser_error::ParserError;
use crate::parser::parser_config::ParserConfig;
use crate::parser::utils::parse_span::ParseSpan;

pub struct ParsingContext<'c, 'p, 'a, 's, 'f> {
    pub config: &'c ParserConfig,

    pub span: &'p ParseSpan<'a,'s,'f>,
}

impl<'c, 'p, 'a, 's, 'f> ParsingContext<'c, 'p, 'a, 's, 'f> {
    pub fn with_different_span(
        &self,
        new_span: &'p ParseSpan<'a,'s,'f>
    ) -> Self {
        Self {
            config: self.config,
            span: new_span,
        }
    }
}

pub trait Parseable
    where Self: Sized + 'static
{
    fn parse(context: &ParsingContext) -> Result<Self, ParserError>;
}

pub trait ParseableWithUnknownSpan
    where Self: Sized + 'static
{
    fn parse_with_unknown_span<'c, 'p, 'a, 's, 'f>(context: &ParsingContext<'c, 'p, 'a, 's, 'f>) -> Result<(Self, ParseSpan<'a,'s,'f>), ParserError>;
}
