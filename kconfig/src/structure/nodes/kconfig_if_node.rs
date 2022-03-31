use crate::structure::kconfig_expression::KconfigExpression;
use crate::structure::kconfig_node_children::KconfigNodeChildren;
use uuid::Uuid;

pub struct KconfigIfNode {
    pub id: Uuid,
    pub condition: KconfigExpression,
    pub children: KconfigNodeChildren,
}
