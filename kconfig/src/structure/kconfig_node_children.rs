use crate::structure::kconfig_node::KconfigNode;

#[derive(Debug, Clone)]
pub struct KconfigNodeChildren {
    pub children: Vec<KconfigNode>,
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

    pub(crate) fn add_all_children(&mut self, children: KconfigNodeChildren) {
        for chd in children.children {
            self.children.push(chd);
        }
    }
}
