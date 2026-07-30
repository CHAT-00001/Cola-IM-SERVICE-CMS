// cola_data/src/three/vo/categoryo.rs  -- 数据中心 - THREE - VO - 分类
// 2026/6/19 17:35

////////

use crate::user::vo::user::UserVo;
use serde::{Deserialize, Serialize};
use crate::three::info::server_type::{ServerTypeInfo};

////////

/// # [VO] - 第三方服务 分类 视图 结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CategoryVo {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub sort: i16,
    pub status: i16,
}

// 构造实现
impl From<ServerTypeInfo> for CategoryVo {
    fn from(info: ServerTypeInfo) -> Self {
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