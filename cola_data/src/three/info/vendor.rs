// cola_data/src/three/info/vendor  -- THREE - 厂商信息
// 2026/6/18

use serde::{Deserialize, Serialize};
use crate::three::entity::vendor::ThreeVendorEntity;

//////

/// # [INFO] - 厂商信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VendorInfo {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub sort: i16,
    pub status: i16,
}

impl From<ThreeVendorEntity> for VendorInfo {
    fn from(e: ThreeVendorEntity) -> Self {
        Self {
            id: e.id,
            code: e.code,
            name: e.name,
            sort: e.sort,
            status: e.status,
        }
    }
}
