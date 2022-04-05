pub mod kconfig_help_property_parser;
pub mod kconfig_menu_node_parser;
pub mod kconfig_node_children_parser;
pub mod parser_traits;
pub mod kconfig_dependency_parser;
pub mod kconfig_if_node_parser;
pub mod source_line_parser;
pub mod kconfig_config_parser;
pub mod kconfig_config_node_parser;
pub mod kconfig_type_property_parser;
pub mod kconfig_reverse_dependency_parser;
pub mod kconfig_menu_config_node_parser;

use crate::parser::kconfig_parser::KconfigParser;
use crate::parser::kconfig_parser_impl::parser_traits::{Parseable, ParsingContext};
use crate::parser::kconfig_parser_state::Parsing;
use crate::parser::utils::parse_span::ParseSpan;
use crate::structure::kconfig::Kconfig;
use crate::structure::kconfig_node_children::KconfigNodeChildren;

impl KconfigParser<Parsing> {
    pub fn parse(&mut self) {
        let lines = self.top_kconfig_source.lines().collect::<Vec<&str>>();
        let span = ParseSpan::from_source(
            &lines[..],
            &self.top_kconfig_path,
        );
        let top_context = ParsingContext {
            config: &self.config,
            span: &span,
        };

        self.result = Some(KconfigNodeChildren::parse(
            &top_context,
        ).map(|children| Kconfig {
            children,
        }));
    }
}
