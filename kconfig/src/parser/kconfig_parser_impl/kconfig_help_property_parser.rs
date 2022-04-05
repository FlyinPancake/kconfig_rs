use crate::errors::parser_error::ParserError;
use crate::parser::kconfig_parser_impl::parser_traits::{ParseableWithUnknownSpan, ParsingContext};
use crate::parser::utils::get_line_indent::get_line_indent;
use crate::parser::utils::parse_span::ParseSpan;
use crate::structure::property::KconfigHelpProperty;

/*

	bool "TI Innovator"
	depends on ARCH_OMAP1 && (ARCH_OMAP15XX || ARCH_OMAP16XX)
	help
          TI OMAP 1510 or 1610 Innovator board support. Say Y here if you
          have such a board.

config MACH_OMAP_H2
	bool "TI H2 Support"
	depends on ARCH_OMAP1 && ARCH_OMAP16XX
    	help
	  TI OMAP 1610/1611B H2 board support. Say Y here if you have such
	  a board.

config MACH_OMAP_H3
	bool "TI H3 Support"
	depends on ARCH_OMAP1 && ARCH_OMAP16XX
    	help
	  TI OMAP 1710 H3 board support. Say Y here if you have such
	  a board.

config asd
    help
config asd


*/

impl ParseableWithUnknownSpan for KconfigHelpProperty {
    fn parse_with_unknown_span<'c, 'p, 'a, 's, 'f>(
        context: &ParsingContext<'c, 'p, 'a, 's, 'f>,
    ) -> Result<(Self, ParseSpan<'a, 's, 'f>), ParserError> {
        let span = context.span;
        span.non_empty_or()?;

        let mut help_text = String::new();
        let help_ident = get_line_indent(span.get_source_span()[0]);
        let mut last_line_index = 0;
        let mut to_match_indent = None;

        for line_index in 1..span.len() {
            let line = span.get_source_span()[line_index];
            let ident = get_line_indent(line);

            if to_match_indent.is_none() && !line.is_empty()  {
                if ident < help_ident && span.get_source_span()[line_index-1].is_empty() {
                    break;
                }
                to_match_indent = Some(ident);
            }

            if ident < to_match_indent.unwrap_or(0) && !line.is_empty() {
                break;
            }

            help_text += line.trim();
            help_text += "\n";
            last_line_index = line_index;
        }

        Ok((
            KconfigHelpProperty::from_text(help_text.trim().to_string()),
            span.get_with_end_at(last_line_index),
        ))
    }
}

#[cfg(test)]
mod test {
    use crate::parser::kconfig_parser_impl::parser_traits::{ParseableWithUnknownSpan, ParsingContext};
    use crate::parser::utils::parse_span::ParseSpan;
    use crate::structure::property::KconfigHelpProperty;

    #[test]
    fn happy_path_help_parsing() {
        let source = "\thelp\n\
        \t\tendmenu lol\n\
        \t\tkeke sajt\n\
        \tdepline\n\
        \tdepline\n\
        ";
        let lines_iter = source.lines().collect::<Vec<&str>>();
        let span = ParseSpan::from_source(&lines_iter[..], "test");
        let context = ParsingContext {
            config: &Default::default(),
            span: &span,
        };

        let parse_res = KconfigHelpProperty::parse_with_unknown_span(&context)
            .unwrap();
        assert_eq!(parse_res.1.get_global_span().1, 2);
        assert_eq!(parse_res.0.help_text, "endmenu lol\nkeke sajt")
    }
}
