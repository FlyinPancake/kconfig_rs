use crate::structure::kconfig_node_children::KconfigNodeChildren;
use crate::structure::property::KconfigDependenciesProperty;

pub struct KconfigMenuNode {
    pub name: String,
    pub dependencies: KconfigDependenciesProperty,
    pub(crate) children: KconfigNodeChildren,
}

impl KconfigMenuNode {
    pub(crate) fn new_empty() -> Self {
        Self {
            name: "".to_string(),
            dependencies: KconfigDependenciesProperty::new_empty(),
            children: KconfigNodeChildren::new_empty(),
        }
    }
}
