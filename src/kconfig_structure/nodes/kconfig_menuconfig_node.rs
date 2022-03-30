use crate::kconfig_structure::kconfig_config::KconfigConfig;
use crate::kconfig_structure::kconfig_node_children::KconfigNodeChildren;
use uuid::Uuid;

pub struct KconfigMenuConfigNode {
    pub id: Uuid,
    pub(crate) children: KconfigNodeChildren,
    config: KconfigConfig,
}
