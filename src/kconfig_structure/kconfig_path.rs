use crate::kconfig_structure::kconfig_node::KconfigNode;
use uuid::Uuid;

pub struct KconfigPath {
    path: Vec<Uuid>,
}

pub trait Traversable {
    fn get_mut_node_at(&mut self, path: &[Uuid]) -> Option<&mut KconfigNode>;
    fn get_node_at(&self, path: &[Uuid]) -> Option<&KconfigNode>;
}
