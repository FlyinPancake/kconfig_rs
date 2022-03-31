use crate::parser::kconfig_parser_impl::kconfig_help_property_parser::parse_and_span_kconfig_help_property;
use crate::parser::utils::rdp::ParseSpan;
use crate::parser::utils::tokenizer::LineKConfigTokenizerIterator;
use crate::structure::property::is_keyword_help_keyword;

pub fn find_index_of_next_keyword_in_span(keyword: &str, span: ParseSpan) -> Option<usize> {
    if span.non_empty_or().is_err() {
        return None;
    }

    let mut line_index = 0;
    let mut line = span.source_span[0];

    loop {
        let mut line_tokens = LineKConfigTokenizerIterator::from_line(line);

        if let Some(first_token) = line_tokens.next() {
            if is_keyword_help_keyword(first_token) && !is_keyword_help_keyword(keyword) {
                let (_, help_span_till) = parse_and_span_kconfig_help_property(span.get_with_start_at(line_index))
                    .ok()?;
                line_index = line_index + help_span_till;
            } else if first_token == keyword {
                return Some(line_index);
            }
        }

        line_index += 1;
        if line_index >= span.len() {
            break;
        }
        line = span.source_span[line_index];
    }

    None
}

#[cfg(test)]
mod test {
    use crate::parser::utils::find_index_of_next_keyword_in_span::find_index_of_next_keyword_in_span;
    use crate::parser::utils::rdp::ParseSpan;

    #[test]
    fn finds_next_menu_end() {
        let source = "menu\n\
        \tasdasdasd\n\
        \thelp\n\
        \t\tendmenu lol\n\
        \t\tkeke sajt\n\
        endmenu\n\
        ";
        let lines_iter = source.lines()
            .collect::<Vec<&str>>();
        let span = ParseSpan::new(&lines_iter[..]);
        assert_eq!(
            find_index_of_next_keyword_in_span(
                "endmenu",
                span,
            ),
            Some(5)
        );
    }
}