// port/src/music/album/manage.rs
// ⏩️ 端口 - 可乐音乐 - 添加
// 2026/8/24 12:14 Created.

////////

use chrono::{DateTime, Utc};
use cola_data::music::info::album::MusicAlbumInfo;
use std::sync::Arc;

////////

/// # [MUSIC ALBUM MANAGE PORTS] - 发布
#[async_trait::async_trait]
pub trait MusicAlbumManagePort: Send + Sync {
    //

    ////////

    /// # 3. [PORT] - 🎶 管理员列表
    async fn get_mange_list(
        &self,
        uid: i64,                          // 操作者 ID
        user_id: Option<i64>,              // 用户 ID
        cate_id: Option<i64>,              // 分类 ID
        channel_id: Option<i64>,           // 频道 ID
        keyword: Option<String>,           // 关键词
        start_time: Option<DateTime<Utc>>, // 开始时间
        end_time: Option<DateTime<Utc>>,   // 结束时间
        limit: i64,                        // 数量
        offset: i64,                       // 页码
    ) -> anyhow::Result<(Vec<MusicAlbumInfo>)> {
        Err(anyhow::anyhow!("管理员视角列表"))
    }
}

//////// END
