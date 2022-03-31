use crate::structure::kconfig_symbol::KconfigSymbol;

pub struct KconfigExpression {
    pub source: String,
    pub included_symbols: Vec<KconfigSymbol>,
}
