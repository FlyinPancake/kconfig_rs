use crate::errors::parser_error::ParserError;
use crate::parser::constants::IF_KEYWORD;
use crate::parser::kconfig_parser_impl::parser_traits::{LineParsingContext, ParseableFromLine};
use crate::parser::utils::tokenizer::LineKConfigTokenizerIterator;
use crate::structure::atoms::{KconfigDependency};
use crate::structure::property::{ConfigType, KconfigTypeProperty};

impl ParseableFromLine for KconfigTypeProperty {
    fn parse_from_line(context: &LineParsingContext) -> Result<Self, ParserError> {
        let mut tokens = LineKConfigTokenizerIterator::from_line(
            context.line.get_line(),
        );

        let keyword = tokens.next()
            .ok_or(
                ParserError::syntax_in_line_span(
                    "Expected type keyword",
                    &context.line,
                ),
            )?;

        let mut if_dep_on_type: Option<KconfigDependency> = None;

        loop {
            if let Some(token) = tokens.next() {
                if token != IF_KEYWORD {
                    continue;
                }
                if_dep_on_type = Some(
                    KconfigDependency::from_source(tokens.get_remaining_slice().trim()),
                );
            } else {
                break;
            }
        }

        Ok(KconfigTypeProperty {
            config_type: ConfigType::from_keyword(keyword)
                .ok_or(
                    ParserError::syntax_in_line_span(
                        "Expected valid type keyword",
                        &context.line,
                    ),
                )?,
            if_dep_on_type,
        })
    }
}