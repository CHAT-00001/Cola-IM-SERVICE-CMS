// cola_data/src/cola_user/entity/role.rs
// 数字中心 - USER - entity - 角色
// 2026/8/3 14:45 Created.

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 用户 - 角色表
/// * `pg schema`: `cola_user`
/// * `table name`: `role`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct UserRoleEntity {
    pub id: i64,                           // id
    pub uid: i64,                          // 操作者用户ID
    pub icon: Option<String>,              // 图标
    pub name: Option<String>,              // 英文名称
    pub name_zh: Option<String>,           // 中文名称
    pub remark: Option<String>,            // 备注
    pub is_deleted: bool,                  // 是否删除: 默认false
    pub status: i16,                       // 状态码: 0无效 1有效
    pub add_time: i64,                     // 添加时间（机器）
    pub upd_time: i64,                     // 更新时间（机器）
    pub created_at: Option<DateTime<Utc>>, // 创建时间（人类）
    pub updated_at: Option<DateTime<Utc>>, // 更新时间（人类）
    pub deleted_at: Option<DateTime<Utc>>, // 删除时间（人类）
}

////////

/// # [COLUMNS] - 数据表原始字段
/// * `desc`: `给SQLx提供的表字段映射`
pub const USER_ROLE_COLUMNS: &str = r#"
    id, uid, icon, name, remark, is_deleted, status,
    addtime, updtime, created_at, updated_at, deleted_at
"#;

//////// END
