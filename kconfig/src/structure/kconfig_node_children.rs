use crate::structure::kconfig_node::KconfigNode;

pub struct KconfigNodeChildren {
    children: Vec<KconfigNode>,
}

impl KconfigNodeChildren {
    pub(crate) fn new_empty() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    pub(crate) fn add_children(&mut self, node: KconfigNode) {
        self.children.push(node);
    }
}
