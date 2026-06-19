// cola_three/src/model/vo/three_biz_binding.rs  -- 绑定 VO
// 2026/6/18

use serde::{Deserialize, Serialize};

//////

/// # [VO] - 业务绑定
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BindingVO {
    pub id: i64,
    pub three_config_id: i64,
    pub biz_module: String,
    pub biz_type: String,
    pub status: i16,
}

impl From<cola_data::three::info::three_biz_binding::BindingInfo> for BindingVO {
    fn from(info: cola_data::three::info::three_biz_binding::BindingInfo) -> Self {
        Self {
            id: info.id,
            three_config_id: info.three_config_id,
            biz_module: info.biz_module,
            biz_type: info.biz_type,
            status: info.status,
        }
    }
}
