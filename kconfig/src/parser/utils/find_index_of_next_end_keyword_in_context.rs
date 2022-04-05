use crate::parser::kconfig_parser_impl::parser_traits::{ParseableWithUnknownSpan, ParsingContext};
use crate::parser::utils::tokenizer::LineKConfigTokenizerIterator;
use crate::structure::property::{is_keyword_help_keyword, KconfigHelpProperty};

pub fn find_index_of_next_end_keyword_in_context(keyword: &str, end_keyword: &str, context: &ParsingContext) -> Option<usize> {
    let span = context.span;
    if span.non_empty_or().is_err() {
        return None;
    }

    let mut line_index = 0;
    let mut line = context.span.get_source_span()[0];
    let mut keyword_count = 0;

    loop {
        let mut line_tokens = LineKConfigTokenizerIterator::from_line(line);

        if let Some(first_token) = line_tokens.next() {
            if is_keyword_help_keyword(first_token) && !is_keyword_help_keyword(keyword) {
                let help_max_span = span.get_with_start_at(line_index);
                let (_, help_span) = KconfigHelpProperty::parse_with_unknown_span(
                    &context.with_different_span(
                        &help_max_span,
                    ),
                ).ok()?;
                line_index += help_span.len() - 1;
            } else if first_token == end_keyword {
                keyword_count -= 1;
                if keyword_count == 0 {
                    return Some(line_index);
                }
            } else if first_token == keyword {
                keyword_count += 1;
            }
        }

        line_index += 1;
        if line_index >= span.len() {
            break;
        }
        line = span.get_source_span()[line_index];
    }

    None
}

#[cfg(test)]
mod test {
    use crate::parser::kconfig_parser_impl::parser_traits::ParsingContext;
    use crate::parser::utils::find_index_of_next_end_keyword_in_context::find_index_of_next_end_keyword_in_context;
    use crate::parser::utils::parse_span::ParseSpan;

    #[test]
    fn finds_next_menu_end() {
        let source = "menu\n\
        \tasdasdasd\n\
        \tmenu\n\
        \tendmenu\n\
        \thelp\n\
        \t\tendmenu lol\n\
        \t\tkeke sajt\n\
        endmenu\n\
        ";
        let lines_iter = source.lines().collect::<Vec<&str>>();
        let span = ParseSpan::from_source(&lines_iter[..], "test");
        let context = ParsingContext {
            config: &Default::default(),
            span: &span,
        };

        assert_eq!(
            find_index_of_next_end_keyword_in_context("menu","endmenu", &context),
            Some(7)
        );
    }
}
