// repo_adapter/src/market/comment/stat.rs
// 🔌 适配器 - MARKET - 商品评论 - 统计
// 2026/8/6 19:18 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::market::comment::stat::GoodsCommentStatPort;

////////

/// # [STAT ADAPTER] - 统计
/// * `desc`: `MARKET - 商品评论统计适配器`
#[derive(Debug, Default, Clone)]
pub struct GoodsCommentStatAdapter;

#[async_trait]
impl GoodsCommentStatPort for GoodsCommentStatAdapter {
    async fn stat_count_by_user_id(&self, uid: i64, user_id: i64) -> Result<(u64)> {
        todo!()
    }

    async fn stat_count_by_video_id(&self, uid: i64, video_id: i64) -> Result<(u64)> {
        todo!()
    }
}
