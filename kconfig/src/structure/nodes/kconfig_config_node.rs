use crate::structure::atoms::KconfigSymbol;
use crate::structure::kconfig_config::KconfigConfig;

#[derive(Debug, Clone)]
pub struct KconfigConfigNode {
    pub symbol: KconfigSymbol,
    pub config: KconfigConfig,
}
