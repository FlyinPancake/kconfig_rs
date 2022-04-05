use crate::errors::parser_error::ParserError;
use crate::parser::utils::parse_span::ParseSpan;
use crate::parser::utils::tokenizer::LineKConfigTokenizerIterator;
use crate::structure::atoms::KconfigSymbol;

pub fn parse_node_header_and_get_name(
    span: &ParseSpan,
    expected_keyword: &str,
) -> Result<String, ParserError> {
    let header_line = span.get_source_span()[0];
    let mut header_tokens = LineKConfigTokenizerIterator::from_line(header_line);

    if !header_tokens.next()
        .contains(&expected_keyword) {
        return Err(ParserError::syntax_in_span_at(
            &format!("Expected {} keyword", expected_keyword),
            &span,
            0,
        ));
    }

    Ok(header_tokens.next()
        .ok_or(ParserError::syntax_in_span_at(
            &format!("Expected {} name", expected_keyword),
            &span,
            0,
        ))?
        .to_string())
}

#[cfg(test)]
mod test {
    use crate::test_utils::make_testing_parsing_contest::make_testing_parsing_contest;

    #[test]
    fn happy_path_parses_menu_header() {
        let context = make_testing_parsing_contest(

        )
    }

}