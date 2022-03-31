use crate::structure::kconfig_node_children::KconfigNodeChildren;
use uuid::Uuid;

pub struct KconfigMenuNode {
    //TODO menu dependencies
    pub id: Uuid,
    pub(crate) children: KconfigNodeChildren,
}
