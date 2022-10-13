use crate::structure::atoms::KconfigReverseDependency;

#[derive(Debug, Clone, Hash)]
pub struct KconfigReverseDependenciesProperty {
    pub dependencies: Vec<KconfigReverseDependency>,
}

impl KconfigReverseDependenciesProperty {
    pub(crate) fn new_empty() -> Self {
        Self {
            dependencies: vec![],
        }
    }

    pub fn add_reverse_dependency(&mut self, dep: KconfigReverseDependency) {
        self.dependencies.push(dep);
    }
}
