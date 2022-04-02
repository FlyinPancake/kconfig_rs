use crate::structure::atoms::kconfig_dependency::KconfigDependency;

pub struct KconfigDependenciesProperty {
    pub dependencies: Vec<KconfigDependency>,
}

impl KconfigDependenciesProperty {
    pub(crate) fn new_empty() -> Self {
        Self {
            dependencies: vec![],
        }
    }

    pub fn add_dependency(
        &mut self,
        dep: KconfigDependency,
    ) {
        self.dependencies.push(dep);
    }
}