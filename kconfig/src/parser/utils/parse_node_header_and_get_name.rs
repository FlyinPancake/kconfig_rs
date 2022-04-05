use crate::errors::parser_error::ParserError;
use crate::parser::utils::parse_span::{LineSpan};
use crate::parser::utils::tokenizer::LineKConfigTokenizerIterator;

pub fn parse_node_header_and_get_name(
    span: &LineSpan,
    expected_keyword: &str,
) -> Result<String, ParserError> {
    let header_line = span.get_line();
    let mut header_tokens = LineKConfigTokenizerIterator::from_line(header_line);

    if !header_tokens.next()
        .contains(&expected_keyword) {
        return Err(ParserError::syntax_in_line_span(
            &format!("Expected {} keyword", expected_keyword),
            &span,
        ));
    }

    Ok(header_tokens.next()
        .ok_or(ParserError::syntax_in_line_span(
            &format!("Expected {} name", expected_keyword),
            &span,
        ))?
        .to_string())
}

#[cfg(test)]
mod test {
    use crate::parser::constants::MENU_KEYWORD;
    use crate::parser::utils::parse_node_header_and_get_name::parse_node_header_and_get_name;
    use crate::test_utils::make_testing_parsing_context::make_testing_parsing_context;

    #[test]
    fn happy_path_parses_menu_header() {
        let context = make_testing_parsing_context(
        "menu \"Keksajtos kifliallarc\"\n\
            \tdepends on SAJTOS && KIFLI\n\
            endmenu\n\
            ",
            Default::default(),
        );

        let menu_name = parse_node_header_and_get_name(
            &context.span.get_line_span_at(0),
            MENU_KEYWORD,
        ).unwrap();

        assert_eq!(menu_name, "\"Keksajtos kifliallarc\"");
    }

}