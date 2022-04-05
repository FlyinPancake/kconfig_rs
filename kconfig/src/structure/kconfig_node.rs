use crate::structure::nodes::{
    KconfigConfigNode, KconfigIfNode, KconfigMenuConfigNode, KconfigMenuNode,
};

#[derive(Debug, Clone)]
pub enum KconfigNode {
    Config(KconfigConfigNode),
    Menu(KconfigMenuNode),
    MenuConfig(KconfigMenuConfigNode),
    If(KconfigIfNode),
}
