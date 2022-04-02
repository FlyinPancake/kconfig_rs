use crate::errors::parser_error::ParserError;
use crate::parser::constants::{MENU_KEYWORD, DEPENDS_KEYWORD, VISIBLE_KEYWORD};
use crate::parser::kconfig_parser_impl::parser_traits::{Parseable, ParseableFromLine, ParsingContext};
use crate::parser::utils::tokenizer::LineKConfigTokenizerIterator;
use crate::structure::atoms::kconfig_dependency::KconfigDependency;
use crate::structure::kconfig_node_children::KconfigNodeChildren;
use crate::structure::nodes::KconfigMenuNode;

fn get_empty_menu_node_from_header(
    context: &ParsingContext
) -> Result<KconfigMenuNode, ParserError> {
    let span = context.span;
    let header_line = span.get_source_span()[0];
    let mut header_tokens = LineKConfigTokenizerIterator::from_line(header_line);

    let no_header_error = ParserError::syntax_in_span_at("Expected menu keyword", &span, 0);
    let header_keyword = header_tokens.next()
        .ok_or(no_header_error.clone())?;
    if header_keyword != MENU_KEYWORD {
        return Err(no_header_error);
    }

    let mut menu_node = KconfigMenuNode::new_empty();
    menu_node.name = header_tokens.next()
        .ok_or(ParserError::syntax_in_span_at("Expected header name", &span, 0))?
        .to_string();

    Ok(menu_node)
}

fn set_menu_properties_and_get_properties_end(
    context: &ParsingContext,
    menu_node: &mut KconfigMenuNode
) -> Result<usize, ParserError> {
    let property_span = context.span.get_with_bounds(1, context.span.len() - 1);
    let mut prop_end = 0;
    for (line_index, property_line) in property_span
        .get_source_span()
        .iter()
        .enumerate() {
        let mut property_line_tokens = LineKConfigTokenizerIterator::from_line(*property_line);
        if let Some(keyword) = property_line_tokens.next() {
            match keyword {
                DEPENDS_KEYWORD => {
                    let line_span = property_span.get_line_span_at(
                      line_index,
                    );
                    let depends = KconfigDependency::parse_from_line(
                        &context.get_line_context_with_span(&line_span),
                    )?;
                    menu_node.dependencies.add_dependency(depends);
                },
                VISIBLE_KEYWORD => {},
                _ => {
                    prop_end = line_index;
                    break;
                }
            }
        } else {
            continue;
        }
    }

    Ok(prop_end + 1)
}

impl Parseable for KconfigMenuNode {
    fn parse(context: &ParsingContext) -> Result<Self, ParserError> {
        let span = context.span;
        span.non_empty_or()?;
        let mut menu_node = get_empty_menu_node_from_header(
            context
        )?;

        let prop_end =
            set_menu_properties_and_get_properties_end(context, &mut menu_node)?;
        let child_span = span.get_with_bounds(prop_end, span.len()-2);
        let node_child = KconfigNodeChildren::parse(
            &context.with_different_span(&child_span),
        )?;
        menu_node.children = node_child;

        Ok(menu_node)
    }
}

#[cfg(test)]
mod test {
    use crate::parser::kconfig_parser_impl::parser_traits::{Parseable, ParsingContext};
    use crate::parser::utils::parse_span::ParseSpan;
    use crate::structure::nodes::KconfigMenuNode;

    #[test]
    fn happy_path_menu_parsing() {
        let source = "menu \"Keksajtos kifliallarc\"\n\
        \tdepends on SAJTOS && KIFLI\n\
        endmenu\n\
        ";
        let lines_iter = source.lines().collect::<Vec<&str>>();
        let span = ParseSpan::from_source(&lines_iter[..], "test");
        let context = ParsingContext {
            config: &Default::default(),
            span: &span,
        };

        let menu_node = KconfigMenuNode::parse(&context).unwrap();
        assert_eq!(menu_node.name, "\"Keksajtos kifliallarc\"");
        assert_eq!(menu_node.dependencies.dependencies.len(), 1);
    }
}