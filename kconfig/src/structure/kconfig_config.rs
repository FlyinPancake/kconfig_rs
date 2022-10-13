use crate::structure::property::{
    KconfigDependenciesProperty, KconfigHelpProperty, KconfigReverseDependenciesProperty,
    KconfigTypeProperty,
};

#[derive(Debug, Clone, Hash)]
pub struct KconfigConfig {
    pub type_property: Option<KconfigTypeProperty>,
    pub dependencies: KconfigDependenciesProperty,
    pub reverse_dependencies: KconfigReverseDependenciesProperty,

    pub help_property: Option<KconfigHelpProperty>,
}

impl KconfigConfig {
    pub fn new_empty() -> Self {
        Self {
            type_property: None,
            help_property: None,
            dependencies: KconfigDependenciesProperty::new_empty(),
            reverse_dependencies: KconfigReverseDependenciesProperty::new_empty(),
        }
    }
}
