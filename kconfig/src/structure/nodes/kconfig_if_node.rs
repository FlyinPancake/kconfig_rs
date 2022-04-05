use crate::structure::atoms::KconfigExpression;
use crate::structure::kconfig_node_children::KconfigNodeChildren;

#[derive(Debug, Clone)]
pub struct KconfigIfNode {
    pub condition: KconfigExpression,
    pub children: KconfigNodeChildren,
}
