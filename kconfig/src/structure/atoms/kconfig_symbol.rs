#[derive(Clone, Debug, PartialEq, Hash)]
pub struct KconfigSymbol {
    pub name: String,
}

impl KconfigSymbol {
    pub fn new(name: String) -> Self {
        KconfigSymbol { name }
    }
}
