use crate::errors::parser_error::ParserError;
use crate::parser::utils::get_line_indent::get_line_indent;
use crate::parser::utils::parse_span::ParseSpan;
use crate::structure::property::KconfigHelpProperty;

pub fn parse_and_span_kconfig_help_property(
    span: ParseSpan,
) -> Result<(KconfigHelpProperty, usize), ParserError> {
    span.non_empty_or()?;
    let mut help_text = String::new();

    let mut last_line_index = 0;
    let mut to_match_indent = None;

    for line_index in 1..span.len() {
        let line = span.get_source_span()[line_index];
        let ident = get_line_indent(line);

        if to_match_indent.is_none() {
            to_match_indent = Some(ident);
        }

        if ident < to_match_indent.unwrap_or(0) {
            break;
        }

        help_text += line.trim();
        help_text += "\n";
        last_line_index = line_index;
    }

    Ok((
        KconfigHelpProperty::from_text(help_text.trim().to_string()),
        last_line_index,
    ))
}
