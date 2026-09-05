// cola_data/src/user/info/role.rs -- 数据 - USER - info - 角色信息
// 2026/8/6 Created.

////////

use serde::{Deserialize, Serialize};

////////

/// # [INFO] - 用户 角色信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RoleInfo {
    pub id: i64,                 // 角色 ID
    pub uid: i64,                // 操作者用户ID
    pub icon: Option<String>,    // 图标
    pub name: Option<String>,    // 英文名称
    pub name_zh: Option<String>, // 中文名称
    pub remark: Option<String>,  // 备注
    pub status: i16,             // 状态码: 0无效 1有效
    pub add_time: i64,           // 添加时间
    pub upd_time: i64,           // 更新时间
}

//////// END
