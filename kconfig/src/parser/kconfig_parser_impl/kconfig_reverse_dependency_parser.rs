use crate::errors::parser_error::ParserError;
use crate::parser::constants::{IF_KEYWORD, SELECT_KEYWORD};
use crate::parser::kconfig_parser_impl::parser_traits::{LineParsingContext, ParseableFromLine};
use crate::parser::utils::tokenizer::LineKConfigTokenizerIterator;
use crate::structure::atoms::{KconfigExpression, KconfigReverseDependency, KconfigSymbol};

fn get_optional_if_constraint(
    tokens: &mut LineKConfigTokenizerIterator,
) -> Option<KconfigExpression> {
    if tokens.next().contains(&IF_KEYWORD) {
        Some(
            KconfigExpression::new(tokens.get_remaining_slice().trim().to_string())
        )
    } else {
        None
    }
}

impl ParseableFromLine for KconfigReverseDependency {
    fn parse_from_line(
        context: &LineParsingContext,
    ) -> Result<Self, ParserError> {
        let mut tokens = LineKConfigTokenizerIterator::from_line(context.line.get_line());

        if !tokens.next()
            .contains(&SELECT_KEYWORD) {
            return Err(ParserError::syntax_in_line_span("Expected select keyword here", &context.line));
        }

        let reverse_dep_symbol = tokens.next()
            .ok_or(
                ParserError::syntax_in_line_span("Expected a symbol here for select", &context.line),
            )?;

        let if_constraint = get_optional_if_constraint(&mut tokens);

        Ok(Self {
            on_symbol: KconfigSymbol::new(reverse_dep_symbol.to_string()),
            if_constraint,
        })
    }
}