use crate::structure::atoms::kconfig_expression::KconfigExpression;
use crate::structure::atoms::KconfigSymbol;

#[derive(Debug, Clone, Hash)]
pub struct KconfigReverseDependency {
    pub on_symbol: KconfigSymbol,
    pub if_constraint: Option<KconfigExpression>,
}
