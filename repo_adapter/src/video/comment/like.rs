// repo_adapter/src/video/comment/like.rs -- ADAPTER - VIDEO - 评论 - 点赞
// 2026/8/6 19:18 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_video::comment::like::VideoCommentLikePort;
use repository::video::pg::comment::like::CommentLikeRepo;
use repository::video::redis::like::CommentLikeRedisRepo;

////////

/// # [LIKE SERVICE] - 视频评论点赞适配器
/// * `desc`: `COLA VIDEO - 视频评论发布服务`
#[derive(Debug, Default, Clone)]
pub struct VideoCommentLikeAdapter;

#[async_trait]
impl VideoCommentLikePort for VideoCommentLikeAdapter {
    //

    ////////

    /// # 1. [SERVICE ADAPTER] - 点赞评论(支持正反操作)
    async fn upsert_like(&self, uid: i64, comment_id: i64, state: bool) -> Result<(bool)> {
        ///////

        // 💡 - PG 仓储
        CommentLikeRepo::update_comment_like_by_id(uid, comment_id, state).await?;

        ////////

        // 💡 - RD 仓储
        CommentLikeRedisRepo::set_relation(uid, comment_id, state).await?;

        // 🗣️ - LOG 日志
        tracing::info!(
            "[🔌 ADAPTER] - ✅️ 评论点赞关系缓存已更新: uid={uid}, comment_id={comment_id}, state={state}"
        );
        Ok(state)
    }

    ////////

    /// # 2. [SERVICE ADAPTER] - 检查状态
    async fn check_state(&self, uid: i64, comment_id: i64) -> Result<(bool)> {
        if let Some(state) = CommentLikeRedisRepo::get_relation(uid, comment_id).await? {
            return Ok(state);
        }

        let state = CommentLikeRepo::check_comment_like_state(uid, comment_id).await?;
        CommentLikeRedisRepo::set_relation(uid, comment_id, state).await?;
        Ok(state)
    }

    ////////

    /// # 3. [SERVICE ADAPTER] - 查询点赞操作幂等结果
    async fn get_operation_result(&self, uid: i64, action_id: &str) -> Result<Option<bool>> {
        CommentLikeRedisRepo::get_operation_result(uid, action_id).await
    }

    ////////

    /// # 4. [SERVICE ADAPTER] - 写入点赞操作幂等结果
    async fn set_operation_result(&self, uid: i64, action_id: &str, state: bool) -> Result<()> {
        CommentLikeRedisRepo::set_operation_result(uid, action_id, state).await
    }
}

//////// END
