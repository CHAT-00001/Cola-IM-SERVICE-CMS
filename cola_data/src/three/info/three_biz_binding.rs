// cola_data/src/three/info/three_biz_binding.rs  -- THREE - 绑定信息
// 2026/6/18

use serde::{Deserialize, Serialize};
use crate::three::entity::three_biz_binding::ThreeBizBindingEntity;

//////

/// # [INFO] - 业务绑定信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BindingInfo {
    pub id: i64,
    pub three_config_id: i64,
    pub biz_module: String,
    pub biz_type: String,
    pub status: i16,
}

impl From<ThreeBizBindingEntity> for BindingInfo {
    fn from(e: ThreeBizBindingEntity) -> Self {
        Self {
            id: e.id,
            three_config_id: e.three_config_id,
            biz_module: e.biz_module,
            biz_type: e.biz_type,
            status: e.status,
        }
    }
}
