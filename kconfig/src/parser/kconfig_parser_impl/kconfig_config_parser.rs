use crate::errors::parser_error::ParserError;
use crate::parser::constants::{DEPENDS_KEYWORD, HELP_KEYWORD, NON_CONFIG_KEYWORDS, SELECT_KEYWORD, TYPE_KEYWORDS};
use crate::parser::kconfig_parser_impl::parser_traits::{ParseableFromLine, ParseableWithUnknownSpan, ParsingContext};
use crate::parser::utils::parse_span::{ParseSpan};
use crate::parser::utils::tokenizer::LineKConfigTokenizerIterator;
use crate::structure::atoms::{KconfigDependency, KconfigReverseDependency};
use crate::structure::kconfig_config::KconfigConfig;
use crate::structure::property::{KconfigHelpProperty, KconfigTypeProperty};

fn is_non_config_keyword(
    keyword: &str,
) -> bool {
    NON_CONFIG_KEYWORDS
        .iter()
        .any(|el| *el == keyword)
}

fn get_type_keyword_from_token(
    token: &str,
) -> Option<&'static str> {
    TYPE_KEYWORDS
        .iter()
        .find(|el| **el == token)
        .map(|el| *el)
}

impl ParseableWithUnknownSpan for KconfigConfig {
    fn parse_with_unknown_span<'c, 'p, 'a, 's, 'f>(
        context: &ParsingContext<'c, 'p, 'a, 's, 'f>
    ) -> Result<(Self, ParseSpan<'a, 's, 'f>), ParserError> {
        let span = context.span;
        let mut line_index = 0;
        let mut config = KconfigConfig::new_empty();

        while line_index < span.len() {
            let line = span.get_source_span()[line_index];
            let mut line_tokens = LineKConfigTokenizerIterator::from_line(line);
            if let Some(first_token) = line_tokens.next() {
                if is_non_config_keyword(first_token) {
                    line_index -= 1;
                    break;
                }
                if get_type_keyword_from_token(first_token).is_some() {
                    let type_prop = KconfigTypeProperty::parse_from_line(
                        &context.get_line_context_with_span(
                            &span.get_line_span_at(line_index),
                        ),
                    )?;
                    if let Some(dep) = type_prop.if_dep_on_type.as_ref() {
                        config.dependencies.add_dependency(dep.clone());
                    }
                    config.type_property = Some(type_prop);
                }

                if first_token == DEPENDS_KEYWORD {
                    let depends = KconfigDependency::parse_from_line(
                        &context.get_line_context_with_span(
                            &span.get_line_span_at(line_index),
                        ),
                    )?;
                    config.dependencies.add_dependency(depends);
                }

                if first_token == SELECT_KEYWORD {
                    let rev_dep = KconfigReverseDependency::parse_from_line(
                        &context.get_line_context_with_span(
                            &span.get_line_span_at(line_index),
                        ),
                    )?;
                    config.reverse_dependencies.add_reverse_dependency(rev_dep);
                }

                if first_token == HELP_KEYWORD {
                    let (help, help_span) = KconfigHelpProperty::parse_with_unknown_span(
                        &context.with_different_span(&context.span.get_with_start_at(line_index)),
                    )?;
                    config.help_property = Some(help);
                    line_index += help_span.len() - 1;
                }
            }

            line_index += 1;
        }

        if line_index == span.len() {
            line_index -= 1;
        }

        Ok((
            config,
            context.span.get_with_end_at(line_index),
        ))
    }
}