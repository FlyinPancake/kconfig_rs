use crate::structure::kconfig_node_children::KconfigNodeChildren;

#[derive(Debug, Clone)]
pub struct Kconfig {
    pub children: KconfigNodeChildren,
}
