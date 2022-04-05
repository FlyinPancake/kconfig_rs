pub const HELP_KEYWORD: &'static str = "help";
pub const HELP_DASHED_KEYWORD: &'static str = "---help---";
pub const HELP_DASHED_PLUS_ONE_KEYWORD: &'static str = "----help----";

pub const MENU_KEYWORD: &'static str = "menu";
pub const END_MENU_KEYWORD: &'static str = "endmenu";

pub const DEPENDS_KEYWORD: &'static str = "depends";
pub const ON_KEYWORD: &'static str = "on";
pub const SELECT_KEYWORD: &'static str = "select";
pub const VISIBLE_KEYWORD: &'static str = "visible";
pub const IF_KEYWORD: &'static str = "if";
pub const ENDIF_KEYWORD: &'static str = "endif";

pub const COMMENT_KEYWORD: &'static str = "comment";

pub const SOURCE_KEYWORD: &'static str = "source";

pub const CONFIG_KEYWORD: &'static str = "config";

pub const MENU_CONFIG_KEYWORD: &'static str = "menuconfig";

// Types
pub const BOOL_KEYWORD: &'static str = "bool";
pub const TRISTATE_KEYWORD: &'static str = "tristate";
pub const STRING_KEYWORD: &'static str = "string";
pub const HEX_KEYWORD: &'static str = "hex";
pub const INT_KEYWORD: &'static str = "int";

pub const TYPE_KEYWORDS: [&'static str; 5] = [
    BOOL_KEYWORD,
    TRISTATE_KEYWORD,
    STRING_KEYWORD,
    HEX_KEYWORD,
    INT_KEYWORD,
];

pub const NON_CONFIG_KEYWORDS: [&'static str; 7] = [
    CONFIG_KEYWORD,
    SOURCE_KEYWORD,
    IF_KEYWORD,
    MENU_KEYWORD,
    ENDIF_KEYWORD,
    END_MENU_KEYWORD,
    MENU_CONFIG_KEYWORD,
];
