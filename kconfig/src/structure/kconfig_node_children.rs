use crate::structure::kconfig_node::KconfigNode;
use crate::structure::kconfig_path::Traversable;
use std::collections::HashMap;
use uuid::Uuid;

pub struct KconfigNodeChildren {
    children: HashMap<Uuid, KconfigNode>,
}

impl Traversable for KconfigNodeChildren {
    fn get_mut_node_at(&mut self, path: &[Uuid]) -> Option<&mut KconfigNode> {
        assert_ne!(path.len(), 0, "Cannot traverse an empty path");
        let next_node_opt = self.children.get_mut(&path[0]);
        if path.len() == 1 {
            next_node_opt
        } else {
            match next_node_opt {
                None => None,
                Some(next_node) => match next_node {
                    KconfigNode::Config(_) => None,
                    KconfigNode::Menu(menu) => menu.children.get_mut_node_at(&path[1..]),
                    KconfigNode::MenuConfig(menu) => menu.children.get_mut_node_at(&path[1..]),
                    KconfigNode::If(kconfig_if) => kconfig_if.children.get_mut_node_at(&path[1..]),
                },
            }
        }
    }

    fn get_node_at(&self, path: &[Uuid]) -> Option<&KconfigNode> {
        assert_ne!(path.len(), 0, "Cannot traverse an empty path");
        let next_node_opt = self.children.get(&path[0]);
        if path.len() == 1 {
            next_node_opt
        } else {
            match next_node_opt {
                None => None,
                Some(next_node) => match next_node {
                    KconfigNode::Config(_) => None,
                    KconfigNode::Menu(menu) => menu.children.get_node_at(&path[1..]),
                    KconfigNode::MenuConfig(menu) => menu.children.get_node_at(&path[1..]),
                    KconfigNode::If(kconfig_if) => kconfig_if.children.get_node_at(&path[1..]),
                },
            }
        }
    }
}

impl KconfigNodeChildren {
    pub(crate) fn new_empty() -> Self {
        Self {
            children: HashMap::new(),
        }
    }

    pub(crate) fn add_children(&mut self, node: KconfigNode) {
        self.children.insert(Uuid::new_v4(), node);
    }
}
