use crate::parser::kconfig_parser_state::{Building, Done, KconfigParserState, Parsing};
use std::marker::PhantomData;
use std::path::Path;
use crate::errors::parser_error::ParserError;
use crate::parser::parser_config::ParserConfig;
use crate::parser::utils::get_string_from_path::get_string_from_path;
use crate::parser::utils::read_file_to_string::read_file_to_string;
use crate::structure::kconfig::Kconfig;

pub struct KconfigParser<State: KconfigParserState> {
    state: PhantomData<State>,

    pub(crate) top_kconfig_path: String,
    pub(crate) top_kconfig_source: String,

    pub(crate) config: ParserConfig,

    pub(crate) result: Option<Result<Kconfig, ParserError>>,
}

impl KconfigParser<Building> {
    pub fn new() -> Self {
        KconfigParser {
            state: Default::default(),
            top_kconfig_path: "<unknown>".to_string(),
            top_kconfig_source: "".to_string(),
            config: Default::default(),
            result: None,
        }
    }

    pub fn set_kconfig_path(mut self, path: &Path) -> Result<Self, ParserError> {
        self.top_kconfig_path = get_string_from_path(path);
        self.top_kconfig_source = read_file_to_string(path)?;
        Ok(self)
    }

    pub fn set_kconfig_source(mut self, source: String) -> Self {
        self.top_kconfig_path = "<unknown>".to_string();
        self.top_kconfig_source = source;

        self
    }

    pub fn allow_sourcing(mut self, allow: bool) -> Self {
        self.config.can_source = allow;

        self
    }

    pub fn set_variable(mut self, variable: &str, value: &str) -> Self {
        self.config.variables.insert(variable.to_string(), value.to_string());

        self
    }

    pub fn parse(self) -> KconfigParser<Done> {
        let mut parsing_parser: KconfigParser<Parsing> = KconfigParser {
            state: Default::default(),
            top_kconfig_path: self.top_kconfig_path,
            top_kconfig_source: self.top_kconfig_source,
            config: self.config,
            result: None,
        };

        parsing_parser.parse();

        KconfigParser {
            state: Default::default(),
            top_kconfig_path: parsing_parser.top_kconfig_path,
            top_kconfig_source: parsing_parser.top_kconfig_source,
            config: parsing_parser.config,
            result: parsing_parser.result,
        }
    }
}

impl KconfigParser<Done> {
    pub fn take_result(self) -> Result<Kconfig, ParserError> {
        self.result.unwrap()
    }
}
