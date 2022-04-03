
#[derive(Debug, Eq, PartialEq)]
pub enum ConfigType {
    Bool,
    Tristate,
    String,
    Hex,
    Int
}

pub struct KconfigTypeProperty {
    pub config_type: ConfigType,
}