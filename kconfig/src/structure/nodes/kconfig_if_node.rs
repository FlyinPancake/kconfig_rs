use crate::structure::atoms::KconfigExpression;
use crate::structure::kconfig_node_children::KconfigNodeChildren;

pub struct KconfigIfNode {
    pub condition: KconfigExpression,
    pub children: KconfigNodeChildren,
}
