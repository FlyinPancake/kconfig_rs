pub use crate::parser::constants::{
    BOOL_KEYWORD,
    TRISTATE_KEYWORD,
    STRING_KEYWORD,
    HEX_KEYWORD,
    INT_KEYWORD,
};
use crate::structure::atoms::KconfigDependency;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConfigType {
    Bool,
    Tristate,
    String,
    Hex,
    Int
}

#[derive(Debug, Clone)]
pub struct KconfigTypeProperty {
    pub config_type: ConfigType,

    pub(crate) if_dep_on_type: Option<KconfigDependency>,
}

impl ConfigType {
    pub fn from_keyword(keyword: &str) -> Option<Self> {
        match keyword {
            BOOL_KEYWORD => Some(ConfigType::Bool),
            TRISTATE_KEYWORD => Some(ConfigType::Tristate),
            STRING_KEYWORD => Some(ConfigType::String),
            HEX_KEYWORD => Some(ConfigType::Hex),
            INT_KEYWORD => Some(ConfigType::Int),
            _ => None,
        }
    }
}