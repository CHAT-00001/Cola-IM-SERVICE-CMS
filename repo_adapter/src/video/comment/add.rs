// repo_adapter/src/video/comment/add.rs  -- ADAPTER - VIDEO - COMMENT - 发布
// 2026/8/8 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::command::comment::CommentCommand;
use cola_data::cola_video::info::comment::CommentInfo;
use port::cola_video::comment::add::VideoCommentAddPort;
use service::cola_video::comment::add::VideoCommentAddService;

////////

/// # [ADD SERVICE] - 视频评论发布适配器
/// * `desc`: `COLA VIDEO - Comment Add Adapter.`
#[derive(Debug, Default, Clone)]
pub struct CommentAddPortAdapter;

#[async_trait]
impl VideoCommentAddPort for CommentAddPortAdapter {

    //

    ////////

    /// # [ADAPTER] - 发布评论
    async fn send_comment(
        &self,
        uid: i64,
        video_id: i64,
        cmd: CommentCommand,
    ) -> Result<(CommentInfo)> {
        let entity = VideoCommentAddService::create_comment(uid, 5, cmd).await?;
        Ok(entity)
    }

    ////////

    /// # [ADAPTER] - 编辑评论
    async fn edit_comment(
        &self,
        uid: i64,
        comment_id: i64,
        cmd: CommentCommand,
    ) -> Result<(CommentInfo)> {
        todo!()
    }
}

//////// END
