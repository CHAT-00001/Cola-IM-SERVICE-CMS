// repo_adapter/src/market/comment/file/del.rs
// 🔌 适配器 - MARKET - 商品评论 - 逻辑删除
// 2026/8/6 19:12 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_video::comment::del::VideoCommentDelPort;
use port::market::comment::del::GoodsCommentDelPort;
////////

/// # [DEL ADAPTER] - 逻辑删除
/// * `desc`: `MARKET - 商品评论逻辑删除适配器`
#[derive(Debug, Default, Clone)]
pub struct GoodsCommentDelAdapter;

#[async_trait]
impl GoodsCommentDelPort for GoodsCommentDelAdapter {

    async fn single_soft_del_record(&self, uid: i64, video_id: i64, id: i64) -> Result<(u16)> {
        todo!()
    }

    async fn batch_soft_del_record(&self, uid: i64, video_id: i64, ids: Vec<i64>) -> Result<(u16)> {
        todo!()
    }
}

//////// END