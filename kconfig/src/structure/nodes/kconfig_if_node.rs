use crate::structure::atoms::kconfig_expression::KconfigExpression;
use crate::structure::kconfig_node_children::KconfigNodeChildren;

pub struct KconfigIfNode {
    pub condition: KconfigExpression,
    pub children: KconfigNodeChildren,
}
