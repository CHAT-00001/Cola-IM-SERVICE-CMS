// repo_adapter/src/market/comment/list.rs
// 🔌 适配器 - MARKET - 商品评论 - 评论列表
// 2026/8/6 18:55 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::comment::VideoCommentInfo;
use port::market::comment::list::GoodsCommentListPort;

////////

/// # [ADD SERVICE] - 发布
/// * `desc`: `可乐视频 - 视频评论发布服务`
#[derive(Debug, Default, Clone)]
pub struct GoodsCommentListAdapter;

#[async_trait]
impl GoodsCommentListPort for GoodsCommentListAdapter {
    async fn get_my_like_record(
        &self,
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(VideoCommentInfo)> {
        todo!()
    }

    async fn get_he_like_record(
        &self,
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(VideoCommentInfo)> {
        todo!()
    }
}
