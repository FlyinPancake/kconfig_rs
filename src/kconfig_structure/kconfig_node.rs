use crate::kconfig_structure::nodes::{
    KconfigConfigNode, KconfigIfNode, KconfigMenuConfigNode, KconfigMenuNode,
};

pub enum KconfigNode {
    Config(KconfigConfigNode),
    Menu(KconfigMenuNode),
    MenuConfig(KconfigMenuConfigNode),
    If(KconfigIfNode),
    // TODO Comment
}
