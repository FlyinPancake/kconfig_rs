use crate::parser::kconfig_parser_state::{Building, Done, KconfigParserState, Parsing};
use std::marker::PhantomData;
use std::path::Path;
use crate::errors::parser_error::ParserError;
use crate::parser::parser_config::ParserConfig;
use crate::parser::utils::read_file_to_string::read_file_to_string;

pub struct KconfigParser<State: KconfigParserState> {
    state: PhantomData<State>,
    pub(crate) top_kconfig_source: String,

    pub(crate) config: ParserConfig,
}

impl KconfigParser<Building> {
    pub fn new() -> Self {
        KconfigParser {
            state: Default::default(),
            top_kconfig_source: "".to_string(),
            config: Default::default(),
        }
    }

    pub fn set_kconfig_path(mut self, path: &Path) -> Result<Self, ParserError> {
        self.top_kconfig_source = read_file_to_string(path)?;
        Ok(self)
    }

    // pub fn set_kconfig_source()

    pub fn parse(mut self) -> KconfigParser<Done> {
        let mut parsing_parser: KconfigParser<Parsing> = KconfigParser {
            state: Default::default(),
            top_kconfig_source: self.top_kconfig_source,
            config: self.config,
        };

        parsing_parser.parse();

        KconfigParser {
            state: Default::default(),
            top_kconfig_source: parsing_parser.top_kconfig_source,
            config: parsing_parser.config,
        }
    }
}
