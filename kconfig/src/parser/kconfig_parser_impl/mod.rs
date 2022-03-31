pub mod kconfig_node_children_parser;
pub mod rdp;

use crate::parser::kconfig_parser::KconfigParser;
use crate::parser::kconfig_parser_state::Parsing;
use crate::structure::kconfig_path::KconfigPath;

impl KconfigParser<Parsing> {
    pub fn parse(&mut self) {
        let top_lines = &self.top_kconfig_source.lines();
    }
}

enum ParsingState {
    ChildrenContext(KconfigPath),
}
