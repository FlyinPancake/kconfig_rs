use crate::structure::property::{KconfigDependenciesProperty, KconfigHelpProperty, KconfigReverseDependenciesProperty, KconfigTypeProperty};

pub struct KconfigConfig {
    pub type_property: KconfigTypeProperty,
    pub dependencies: KconfigDependenciesProperty,
    pub reverse_dependencies: KconfigReverseDependenciesProperty,

    pub help_property: Option<KconfigHelpProperty>,
}
