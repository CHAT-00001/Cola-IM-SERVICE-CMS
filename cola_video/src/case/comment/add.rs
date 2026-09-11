// cola_video/src/case/comment/add.rs -- VIDEO - CASE - 评论发布
// 2026/9/11 Created.

////////

use crate::assembler::comment::build_comment_single_response;
use crate::model::vo::comment::CommentSingleResponse;
use anyhow::Result;
use cola_data::cola_video::command::comment::CommentCommand;
use port::app::ctx::AppContext;
use tracing::info;

////////

/// # [CASE] - 视频评论发布用例
pub struct CommentAddCase;

impl CommentAddCase {
    /// # 1. [CASE] - 发布评论
    /// * `desc`: 调用评论端口持久化并组装单条响应
    pub async fn case_send(
        uid: i64,            // 操作者 ID
        video_id: i64,       // 视频 ID
        cmd: CommentCommand, // 评论命令
        ctx: AppContext,     // 应用上下文
    ) -> Result<CommentSingleResponse> {
        let info = ctx
            .video
            .comment
            .add
            .send_comment(uid, video_id, cmd)
            .await
            .map_err(|error| anyhow::anyhow!("评论发布持久化失败: {error}"))?;
        info!("[🗣️ COMMENT CASE] - ✅️ 评论发布成功: uid={uid}, video_id={video_id}");
        build_comment_single_response(info, Some(uid)).await
    }
}

//////// END