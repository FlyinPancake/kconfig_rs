use crate::structure::atoms::kconfig_expression::KconfigExpression;

#[derive(Clone, Debug, Hash)]
pub struct KconfigDependency {
    pub expression: KconfigExpression,
}

impl KconfigDependency {
    pub fn from_source(expr: &str) -> Self {
        Self {
            expression: KconfigExpression::new(expr.to_string()),
        }
    }
}
