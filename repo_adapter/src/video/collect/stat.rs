// repo_adapter/src/video/collect/stat.rs
// 🔌 适配器 - ▶ 视频 - 收藏 - 统计
// 2026/8/9 20:28 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_video::collect::stat::VideoCollectStatPort;

////////

/// # [STAT ADAPTER] - 统计
/// * `desc`: `▶ 可乐视频 - 统计收藏数量`
#[derive(Debug, Default, Clone)]
pub struct VideoCollectStatAdapter;

#[async_trait]
impl VideoCollectStatPort for VideoCollectStatAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 用户的
    /// * `desc`: `根据用户ID` - `统计用户的收藏数量`
    async fn stat_count_by_user_id(
        &self,
        uid: i64,
        user_id: i64, // 用户 ID
    ) -> anyhow::Result<(u64)> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 视频的
    /// * `desc`: `根据视频ID` - `统计视频的收藏数量`
    async fn stat_count_by_video_id(
        &self,
        uid: i64,
        video_id: i64, // 视频 ID
    ) -> anyhow::Result<(u64)> {
        todo!()
    }
}

//////// END
