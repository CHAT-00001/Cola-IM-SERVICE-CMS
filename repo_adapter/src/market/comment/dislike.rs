// repo_adapter/src/market/comment/dislike.rs
// 🔌 适配器 - MARKET - 商品评论 - 不喜欢
// 2026/8/6 19:58 Created.

////////

use async_trait::async_trait;
use port::market::comment::dislike::GoodsCommentDisikePort;

////////

/// # [DISLIKE ADAPTER] - 评论不喜欢服务
pub struct GoodsCommentDislikeAdapter;

#[async_trait]
impl GoodsCommentDisikePort for GoodsCommentDislikeAdapter {
    //

    ////////

    /// 1. # [SERVICE] 不喜欢/取消
    async fn upsert_dislike(
        &self,
        uid: i64,
        comment_id: i64,
        is_disliked: bool,
    ) -> anyhow::Result<(bool)> {
        todo!()
    }

    async fn check_state(&self, uid: i64, comment_id: i64) -> anyhow::Result<(bool)> {
        todo!()
    }
}

//////// END
