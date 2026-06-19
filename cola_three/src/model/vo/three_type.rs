// cola_three/src/model/vo/three_type.rs  -- 类型 VO
// 2026/6/18

use serde::{Deserialize, Serialize};

//////

/// # [VO] - 服务类型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TypeVO {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub sort: i16,
    pub status: i16,
}

impl From<cola_data::three::info::three_type::TypeInfo> for TypeVO {
    fn from(info: cola_data::three::info::three_type::TypeInfo) -> Self {
        Self {
            id: info.id,
            code: info.code,
            name: info.name,
            sort: info.sort,
            status: info.status,
        }
    }
}
