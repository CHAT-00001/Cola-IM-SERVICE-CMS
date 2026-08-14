// repo_adapter/src/market/comment/add.rs  --
// 🔌 适配器 - MARKET - 商品评论 - 发布
// 2026/8/8 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::command::comment::CommentCommand;
use cola_data::cola_video::info::comment::VideoCommentInfo;
use port::market::comment::add::GoodsCommentAddPort;

////////

/// # [ADD ADAPTER] - 发布
/// * `desc`: `MARKET - 商品评论发布适配器`
#[derive(Debug, Default, Clone)]
pub struct GoodsCommentAddAdapter;

#[async_trait]
impl GoodsCommentAddPort for GoodsCommentAddAdapter {
    async fn send_comment(
        &self,
        uid: i64,
        video_id: i64,
        cmd: CommentCommand,
    ) -> Result<(VideoCommentInfo)> {
        todo!()
    }

    async fn edit_comment(
        &self,
        uid: i64,
        comment_id: i64,
        cmd: CommentCommand,
    ) -> Result<(VideoCommentInfo)> {
        todo!()
    }
}

//////// END
