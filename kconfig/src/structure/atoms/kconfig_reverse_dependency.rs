use crate::structure::atoms::kconfig_expression::KconfigExpression;
use crate::structure::atoms::KconfigSymbol;

pub struct KconfigReverseDependency {
    pub on_symbol: KconfigSymbol,
    pub if_constraint: Option<KconfigExpression>,
}