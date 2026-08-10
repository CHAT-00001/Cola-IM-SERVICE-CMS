// repo_adapter/src/cola_video/recommend/stat.rs
// 🔌 插头 - 可乐视频 - 推荐 - 统计
// 2026/8/6 19:16 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_video::recommend::stat::VdieoRecommendStatPort;

////////

/// # [STAT ADAPTER] - recommend stat
/// * `DESC`: `▶ 视频 - 推荐计数适配器`
#[derive(Debug, Default, Clone)]
pub struct recommendstatPortAdapter;

#[async_trait]
impl VdieoRecommendStatPort for recommendstatPortAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 用户的
    async fn stat_count_by_user_id(&self, uid: i64, user_id: i64) -> Result<(u64)> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 视频的
    async fn stat_count_by_video_id(&self, uid: i64, video_id: i64) -> Result<(u64)> {
        todo!()
    }
}

//////// END
