// cola_data/src/gift/entity/sticker.rs  -- GIFT - entity - 自定义表情贴纸
// 2026/6/19 17:58

////////

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 表情包贴纸 表
/// * `table name`: gift_sticker
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct GiftStickerEntity {
    pub id: i64,         // ID
    pub uid: i64,        // 用户ID
    pub name: String,    // 中文名称
    pub name_en: String, // 英文名称
    pub url: String,     // 地址
    pub sort: i16,       // 排序
    pub status: i16,     // 状态
    pub addtime: i64,    // 添加时间 - 机器
    pub updtime: i64,    // 更新时间 - 机器
}
