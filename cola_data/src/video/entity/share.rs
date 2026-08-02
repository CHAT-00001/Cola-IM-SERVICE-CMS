// cola_data/src/handler/new//share.rs  -- 数据 - handler - 短视频 - 收藏
// 2026/5/20 19:56 by wx: cestbon10080
// * 1个结构体
// * --------

////////

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 视频分享实体
/// * table name: video_biz.video_collect (指定在短视频业务子库下)
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct ShareEntity {
    pub id: i64,              // 分享 ID
    pub user_id: i64,         // 谁分享的
    pub video_id: i64,        // 分享了哪个视频
    pub target_platform: i16, // 分享到了哪里：1. 微信 2. 朋友圈 3. QQ 4. 复制链接
    pub share_code: String,   // 💡 自动生成的唯一分享口令/短码
    pub sync_id: String,      // 同步 id （客户端生成 uuid v4）
    pub sync_time: i64,       // 同步时间
    pub create_time: i64,     // 创建时间 （客户端生成）
}

// * --------
//////// END
