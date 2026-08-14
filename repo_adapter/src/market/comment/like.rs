// repo_adapter/src/market/comment/like.rs
// 🔌 适配器 - MARKET - 商品评论 - 评论点赞
// 2026/8/6 19:18 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::market::comment::like::GoodsCommentLikePort;

////////

/// # [ADD SERVICE] - 发布
/// * `desc`: `可乐视频 - 视频评论发布服务`
#[derive(Debug, Default, Clone)]
pub struct GoodsCommentLikeAdapter;

#[async_trait]
impl GoodsCommentLikePort for GoodsCommentLikeAdapter {
    async fn upsert_like(&self, uid: i64, comment_id: i64, state: bool) -> Result<(bool)> {
        todo!()
    }

    async fn check_state(&self, uid: i64, comment_id: i64) -> Result<(bool)> {
        todo!()
    }
}
