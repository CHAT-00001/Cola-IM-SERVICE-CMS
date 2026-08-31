// repo_adapter/src/cola_video/like/stat.rs
// 🔌 适配器 - 可乐视频 - 点赞 - 统计
// 2026/8/6 19:17 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_video::like::stat::VideoLikeStatPort;

////////

/// # [STAT ADAPTER] - like stat
/// * `desc`: `▶ 视频 - 视频点赞记录统计适配器`
#[derive(Debug, Default, Clone)]
pub struct LikeStatPortAdapter;

#[async_trait]
impl VideoLikeStatPort for LikeStatPortAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 用户的
    async fn stat_count_by_user_id(
        &self,
        uid: i64,
        user_id: i64, // 用户 ID
    ) -> Result<(u64)> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 视频的
    async fn stat_count_by_video_id(
        &self,
        uid: i64,
        video_id: i64, // 视频 ID
    ) -> Result<(u64)> {
        todo!()
    }
}

//////// END
