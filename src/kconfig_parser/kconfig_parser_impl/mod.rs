use crate::kconfig_parser::kconfig_parser::KconfigParser;
use crate::kconfig_parser::kconfig_parser_state::Parsing;
use crate::kconfig_structure::kconfig_path::KconfigPath;

impl KconfigParser<Parsing> {
    pub fn parse(&mut self) {
        let top_lines = &self.top_kconfig_source.lines();
    }
}

enum ParsingState {
    ChildrenContext(KconfigPath),
}
