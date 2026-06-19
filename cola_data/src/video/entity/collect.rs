// cola_data/src/handler/video/collect.rs  -- 数据 - handler - 短视频 - 收藏
// 2026/5/20 19:05 by wx: cestbon10080
// * 1个结构体
// * --------

////////

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 视频收藏实体
/// * table name: video_biz.video_collect (指定在短视频业务子库下)
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct CollectEntity {
    pub id: i64,                // 收藏记录自增 ID
    pub user_id: i64,           // 收藏发起者用户 ID
    pub video_id: i64,          // 被收藏的短视频 ID
    pub folder_id: i64,         // 收藏夹 ID：默认 0 (主收藏夹/未分类)
    pub channel: i16,           // 来源通道：默认 1 (APP), 2 (微信小程序) 等
    pub remark: Option<String>, // 用户对该收藏视频的个性化备注/签注
    pub status: i16,            // 状态：0. 正常收藏 1. 已取消收藏 2. 视频失效隐藏

    // 🕒 时间戳阵列优化（干掉 String，全面迎合 Rust 高性能数值类型）
    pub add_time: i32,          // 创建时间戳：秒级，完美兼容旧版 PHP 历史数据与快速索引
    pub sync_time: i64,         // 服务器同步时间：毫秒级时间戳
    pub create_time: i64,       // 精确创建时间：毫秒级时间戳
    pub update_time: i64,       // 精确更新时间：毫秒级时间戳
}

// * --------
//////// END