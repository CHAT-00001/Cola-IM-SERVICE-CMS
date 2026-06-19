// cola_data/src/three/info/three_type.rs  -- THREE - 类型信息
// 2026/6/18

use serde::{Deserialize, Serialize};
use crate::three::entity::three_type::ThreeTypeEntity;

//////

/// # [INFO] - 服务类型信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TypeInfo {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub sort: i16,
    pub status: i16,
}

impl From<ThreeTypeEntity> for TypeInfo {
    fn from(e: ThreeTypeEntity) -> Self {
        Self {
            id: e.id,
            code: e.code,
            name: e.name,
            sort: e.sort,
            status: e.status,
        }
    }
}
