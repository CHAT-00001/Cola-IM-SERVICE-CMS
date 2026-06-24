// cola_three/src/model/vo/three_vendor.rs  -- 厂商 VO
// 2026/6/18

////////

use serde::{Deserialize, Serialize};

//////

/// # [VO] - 厂商
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VendorVO {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub sort: i16,
    pub status: i16,
}

// 构造函数
impl From<cola_data::three::info::three_vendor::VendorInfo> for VendorVO {
    fn from(info: cola_data::three::info::three_vendor::VendorInfo) -> Self {
        Self {
            id: info.id,
            code: info.code,
            name: info.name,
            sort: info.sort,
            status: info.status,
        }
    }
}

//////// END
