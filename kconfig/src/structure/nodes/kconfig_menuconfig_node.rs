use crate::structure::atoms::KconfigSymbol;
use crate::structure::kconfig_config::KconfigConfig;
use crate::structure::kconfig_node_children::KconfigNodeChildren;

pub struct KconfigMenuConfigNode {
    pub symbol: KconfigSymbol,

    pub children: KconfigNodeChildren,
    pub config: KconfigConfig,
}
