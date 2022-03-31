use crate::errors::parser_error::ParserError;

pub struct ParseSpan<'a, 's> {
    pub source_span: &'a [&'s str],
}

impl<'a, 's> ParseSpan<'a, 's> {
    pub fn new(source_span: &'a [&'s str]) -> Self {
        Self {
            source_span,
        }
    }

    pub fn non_empty_or(&self) -> Result<(), ParserError> {
        if self.source_span.len() == 0 {
            Err(ParserError::Internal("Expected a non empty span.".to_string()))
        } else {
            Ok(())
        }
    }

    pub fn len(&self) -> usize {
        self.source_span.len()
    }

    pub fn get_with_start_at(&self, at: usize) -> ParseSpan {
        ParseSpan::new(&self.source_span[at..])
    }

    pub fn get_with_end_at(&self, at: usize) -> ParseSpan {
        ParseSpan::new(&self.source_span[..at])
    }
}
