// repo_adapter/src/video/hotlist/stat.rs
// 🔌 插头 - ▶ 可乐视频 - 上热门 - 统计
// 2026/8/9 22:56 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_video::hotlist::stat::VideoHotlistStatPort;

////////

/// # [STAT ADAPTER] - hotlist stat
/// * `desc`: `▶ 视频 - 上热门记录统计适配器`
#[derive(Debug, Default, Clone)]
pub struct VideoHotlistStatAdapter;

// 构造实现
#[async_trait]
impl VideoHotlistStatPort for VideoHotlistStatAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 用户的
    async fn stat_count_by_user_id(
        &self,
        uid: i64,     // UID
        user_id: i64, // 用户 ID
    ) -> Result<(u64)> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 视频的
    async fn stat_count_by_video_id(
        &self,
        uid: i64,      // UID
        video_id: i64, // 视频 ID
    ) -> Result<(u64)> {
        todo!()
    }
}

//////// END
