// cola_data/src/im/entity/contact.rs  -- 数据中心 -IM - entity - 联系人
// 2026-07-07 14:30

////////

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// #  [ENTITY] - 联系人 - 实体表结构
/// * `pg schema`: `cola_im`
/// * `table name`: `contact`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct ContactEntity {
    pub id: i64,                     // 联系人 ID
    pub send_id: String,             // 发送 ID
    pub owner_id: i64,               // 用户 ID
    pub group_id: Option<i64>,       // 分组 ID
    pub card_id: i64,                // 对方名片 ID
    pub remark_name: Option<String>, // 备注名称
    pub is_stared: i16,              // 是否星标: 默认0否
    pub favorites: bool,             // 特别关心
    pub blocked: bool,               // 是否已拉黑名单
    pub deleted: bool,               // 是否已删除
    pub sort: Option<i64>,           // 排序
    pub status: i16,                 // 状态码
    pub add_time: i64,               // 添加时间 - 机器
    pub upd_time: i64,               // 更新时间 - 机器
    pub del_time: i64,               // 删除时间 - 机器
}

//////// END
