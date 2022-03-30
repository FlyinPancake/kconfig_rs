use crate::kconfig_structure::kconfig_config::KconfigConfig;
use uuid::Uuid;

pub struct KconfigConfigNode {
    pub id: Uuid,
    pub config: KconfigConfig,
}
