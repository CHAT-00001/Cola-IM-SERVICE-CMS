// repo_adapter/src/market/collec/stat.rs
// 🔌 适配器 - MARKET - 商品收藏 - 统计
// 2026/8/9 20:28 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::market::collect::stat::GoodsCollectStatPort;

////////

/// # [STAT ADAPTER] - 统计
/// * `desc`: `商品收藏统计适配器`
#[derive(Debug, Default, Clone)]
pub struct GoodsCollectStatAdapter;

#[async_trait]
impl GoodsCollectStatPort for GoodsCollectStatAdapter {
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
