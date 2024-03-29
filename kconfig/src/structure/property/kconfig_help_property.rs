use crate::parser::constants::{HELP_DASHED_KEYWORD, HELP_DASHED_PLUS_ONE_KEYWORD, HELP_KEYWORD};

#[derive(Debug, Clone, Hash)]
pub struct KconfigHelpProperty {
    pub help_text: String,
}

impl KconfigHelpProperty {
    pub(crate) fn from_text(help_text: String) -> Self {
        Self { help_text }
    }
}

pub(crate) fn is_keyword_help_keyword(keyword: &str) -> bool {
    keyword == HELP_KEYWORD
        || keyword == HELP_DASHED_KEYWORD
        || keyword == HELP_DASHED_PLUS_ONE_KEYWORD
}
