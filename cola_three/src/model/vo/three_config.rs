// cola_three/src/model/vo/three_config.rs  -- 配置 VO
// 2026/6/18

////////

use serde::{Deserialize, Serialize};

//////

/// # [VO] - 第三方服务配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigVO {
    pub id: i64,
    pub type_id: i64,
    pub vendor_id: i64,
    pub name: String,
    pub bucket: String,
    pub access_key: String,
    pub endpoint: String,
    pub region: String,
    pub config_json: Option<serde_json::Value>,
    pub remark: Option<String>,
    pub status: i16,
}
// 转换函数
impl From<cola_data::three::info::three_config::ConfigInfo> for ConfigVO {
    fn from(info: cola_data::three::info::three_config::ConfigInfo) -> Self {
        Self {
            id: info.id,
            type_id: info.type_id,
            vendor_id: info.vendor_id,
            name: info.name,
            bucket: info.bucket,
            access_key: info.access_key,
            endpoint: info.endpoint,
            region: info.region,
            config_json: info.config_json,
            remark: info.remark,
            status: info.status,
        }
    }
}


//////// END