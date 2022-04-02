pub mod kconfig_help_property_parser;
pub mod kconfig_menu_node_parser;
pub mod kconfig_node_children_parser;
pub mod parser_traits;
pub mod kconfig_dependency_parser;
pub mod kconfig_if_node_parser;

use crate::parser::kconfig_parser::KconfigParser;
use crate::parser::kconfig_parser_state::Parsing;

impl KconfigParser<Parsing> {
    pub fn parse(&mut self) {
        let top_lines = &self.top_kconfig_source.lines();
    }
}
