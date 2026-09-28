// cola_video/src/case/comment/like.rs -- VIDEO - CASE - 评论点赞
// 2026/9/29 03:30 Created.

////////

use anyhow::Result;
use port::app::ctx::AppContext;
use tracing::info;
use uuid::{Uuid, Version};

////////

/// # [CASE] - 视频评论点赞用例
pub struct CommentLikeCase;

impl CommentLikeCase {
    //

    ////////

    /// # 1. [CASE] - 点赞或取消点赞评论
    /// * `desc`: `校验 UUID v4 幂等请求并调用评论点赞端口`
    pub async fn case_set_comment_like(
        uid: i64,          // 操作者 UID
        comment_id: i64,   // 评论 ID
        state: bool,       // 点赞状态
        action_id: String, // 业务动作 UUID v4
        ctx: &AppContext,  // 应用上下文
    ) -> Result<bool> {
        let uuid = Uuid::parse_str(action_id.trim())
            .map_err(|_| anyhow::anyhow!("客户端幂等ID必须是 UUID v4"))?;
        if uuid.get_version() != Some(Version::Random) {
            return Err(anyhow::anyhow!("客户端幂等ID必须是 UUID v4"));
        }
        let action_id = uuid.to_string();

        if let Some(result) = ctx
            .video
            .comment
            .like
            .get_operation_result(uid, &action_id)
            .await?
        {
            info!("[🗣️ COMMENT CASE] - ✅️ 评论点赞幂等命中: uid={uid}, comment_id={comment_id}");
            return Ok(result);
        }

        let result = ctx
            .video
            .comment
            .like
            .upsert_like(uid, comment_id, state)
            .await?;
        ctx.video
            .comment
            .like
            .set_operation_result(uid, &action_id, result)
            .await?;
        info!(
            "[🗣️ COMMENT CASE] - ✅️ 评论点赞完成: uid={uid}, comment_id={comment_id}, state={result}"
        );
        Ok(result)
    }
}

//////// END
