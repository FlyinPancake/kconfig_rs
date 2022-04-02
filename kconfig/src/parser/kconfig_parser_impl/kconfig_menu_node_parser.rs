use crate::errors::parser_error::ParserError;
use crate::parser::utils::parse_span::ParseSpan;
use crate::structure::nodes::KconfigMenuNode;

pub fn parse_kconfig_menu_node(_span: &ParseSpan) -> Result<KconfigMenuNode, ParserError> {
    Err(ParserError::Syntax("unimplemented".to_string()))
}
