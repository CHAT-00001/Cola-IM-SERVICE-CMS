// repo_adapter/src/video/comment/add.rs  --
// 🔌 适配器 - 视频 - 评论 - 发布 服务
// 2026/8/8 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::command::comment::CommentCommand;
use cola_data::cola_video::info::comment::VideoCommentInfo;
use port::cola_video::comment::add::VideoCommentAddPort;

////////

/// # [ADD SERVICE] - 发布
/// * `desc`: `可乐视频 - 视频评论发布服务`
#[derive(Debug, Default, Clone)]
pub struct CommentAddPortAdapter;

#[async_trait]
impl VideoCommentAddPort for CommentAddPortAdapter {
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
