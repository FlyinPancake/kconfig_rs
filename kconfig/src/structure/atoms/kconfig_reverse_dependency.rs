use crate::structure::atoms::kconfig_expression::KconfigExpression;

pub struct KconfigReverseDependency {
    pub expression: KconfigExpression,
    pub if_constraint: Option<KconfigExpression>,
}