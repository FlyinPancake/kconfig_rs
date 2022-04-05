use std::fs::read_to_string;
use std::path::Path;
use crate::errors::parser_error::ParserError;
use crate::parser::constants::SOURCE_KEYWORD;
use crate::parser::kconfig_parser_impl::parser_traits::{LineParsingContext, Parseable, ParsingContext};
use crate::parser::utils::collapse_manual_line_breaks::collapse_manual_line_breaks;
use crate::parser::utils::parse_span::ParseSpan;
use crate::parser::utils::strip_quotes::strip_quotes;
use crate::parser::utils::substitute_variables_in_string::substitute_variables_in_string;
use crate::parser::utils::tokenizer::LineKConfigTokenizerIterator;
use crate::structure::kconfig_node_children::KconfigNodeChildren;

pub fn parse_source_line(context: &LineParsingContext) -> Result<KconfigNodeChildren, ParserError> {
    let line = context.line;
    let mut tokens = LineKConfigTokenizerIterator::from_line(line.get_line());

    if !tokens.next().contains(&SOURCE_KEYWORD) {
        return Err(ParserError::syntax_in_line_span("Expected source keyword", line));
    }

    let line_location_str = format!("at {} line {}", line.get_filename(), line.get_global_at());
    if !context.config.can_source {
        return Err(ParserError::EncounteredDisabledSource(line_location_str));
    }

    let source_path_raw = strip_quotes(&tokens.next()
        .ok_or(ParserError::syntax_in_line_span("Expected source path", line))?);

    let source_path_compiled = substitute_variables_in_string(
        &context.config,
        &source_path_raw,
    );

    let source_final_path =
        Path::new(&context.config.root_path)
            .parent()
            .unwrap_or(Path::new("./"))
            .join(Path::new(&source_path_compiled));

    let source = read_to_string(source_final_path)
        .map_err(|err| ParserError::FileRead(format!("{}, {}", err, line_location_str)))?;
    let source_lines = collapse_manual_line_breaks(
        source.lines().collect()
    );
    let file_contents = source_lines
        .iter()
        .map(|el| el.as_ref())
        .collect::<Vec<&str>>();
    let new_context = ParsingContext {
        config: context.config,
        span: &ParseSpan::from_source(&file_contents[..], &source_path_compiled),
    };

    Ok(KconfigNodeChildren::parse(&new_context)?)
}