use crate::errors::parser_error::ParserError;
use crate::parser::constants::IF_KEYWORD;
use crate::parser::kconfig_parser_impl::parser_traits::{Parseable, ParsingContext};
use crate::parser::utils::tokenizer::LineKConfigTokenizerIterator;
use crate::structure::atoms::kconfig_expression::KconfigExpression;
use crate::structure::kconfig_node_children::KconfigNodeChildren;
use crate::structure::nodes::KconfigIfNode;

impl Parseable for KconfigIfNode {
    fn parse(context: &ParsingContext) -> Result<Self, ParserError> {
        let span = context.span;
        let header_line = span.get_source_span()[0];
        let mut header_tokens = LineKConfigTokenizerIterator::from_line(header_line);

        if !header_tokens.next()
            .contains(&IF_KEYWORD)
        {
            return Err(ParserError::syntax_in_span_at("Expected if keyword", &span, 0));
        }
        let expr_src = header_tokens.get_remaining_slice().trim();
        let expr = KconfigExpression::new(expr_src.to_string());
        let children_span = span.get_with_bounds(1, span.len()-2);
        let children_node = KconfigNodeChildren::parse(&context.with_different_span(&children_span))?;

        Ok(Self {
            condition: expr,
            children: children_node,
        })
    }
}