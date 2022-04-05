use crate::structure::kconfig_node_children::KconfigNodeChildren;
use crate::structure::property::KconfigDependenciesProperty;

#[derive(Debug, Clone)]
pub struct KconfigMenuNode {
    pub name: String,
    pub dependencies: KconfigDependenciesProperty,
    pub children: KconfigNodeChildren,
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
