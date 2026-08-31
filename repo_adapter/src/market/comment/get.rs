// repo_adapter/src/market/comment/get.rs
// 🔌 适配器 - MARKET - 商品评论 - 获取IDs
// 2026/8/6 18:55 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::command::comment::CommentCommand;
use cola_data::cola_video::info::comment::VideoCommentInfo;
use port::market::comment::get::GoodsCommentGetPort;

////////

/// # [GET SERVICE] - 发布
/// * `desc`: `可乐视频 - 视频评论发布服务`
#[derive(Debug, Default, Clone)]
pub struct GoodsCommentLikeAdapter;

#[async_trait]
impl GoodsCommentGetPort for GoodsCommentLikeAdapter {
    async fn save_comment_record(
        &self,
        uid: i64,
        video_id: i64,
        is_liked: bool,
    ) -> Result<(VideoCommentInfo)> {
        todo!()
    }

    async fn edit_comment_record(
        &self,
        comment_id: i64,
        cmd: CommentCommand,
    ) -> Result<(VideoCommentInfo)> {
        todo!()
    }

    async fn del_comment_record(&self, comment_id: i64) -> Result<()> {
        todo!()
    }

    async fn del_comments_record(&self, comment_ids: Vec<i64>) -> Result<()> {
        todo!()
    }
}
