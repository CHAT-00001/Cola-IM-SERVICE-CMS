// servicey/src/cola_video/comment/check.rs
// 服务 - ▶ 可乐视频  - 评论 - 状态检查
// 2026/8/12 05:03 Created.

////////

use repository::cola_video::pg::comment::check::VideoCommentCheckRepo;
use tracing::{error, info};

////////

/// # [CHECK SERVICE] - 评论 检查
/// * `desc`: `检查评论状态`
pub struct CommentCheckService;

impl CommentCheckService {
    ////////

    /// # 1. [SERVICE] - 健康
    pub async fn check_health(_uid: i64, comment_id: i64) -> Result<i16, anyhow::Error> {
        info!(
            "[🗣️ SERVICE]: ✅️ 检查评论健康分, comment_id: {}",
            comment_id
        );

        let result = VideoCommentCheckRepo::find_health(comment_id).await?;

        // 取第一个结果，如果没查到则返回错误
        result.first().copied().ok_or_else(|| {
            error!(
                "[🤐 SERVICE]:  ❌️ 未找到评论健康分, comment_id: {}",
                comment_id
            );
            anyhow::anyhow!("Comment not found")
        })
    }

    ////////

    /// # 2. [SERVICE] - 状态
    pub async fn check_status(comment_id: i64) -> Result<i16, anyhow::Error> {
        info!(
            "[🗣️ SERVICE]: ✅️ 检查评论状态码, comment_id: {}",
            comment_id
        );

        let result = VideoCommentCheckRepo::find_status(comment_id).await?;

        result.first().copied().ok_or_else(|| {
            error!(
                "[🤐 SERVICE]:  ❌️ 未找到评论状态码, comment_id: {}",
                comment_id
            );
            anyhow::anyhow!("Comment not found")
        })
    }

    ////////

    /// # 3. [SERVICE] - 归属
    pub async fn is_owner(comment_id: i64, user_id: i64) -> Result<bool, anyhow::Error> {
        info!(
            "[🗣️ SERVICE]: ✅️ 检查评论归属, comment_id: {}, uid: {}",
            comment_id, user_id
        );

        let result = VideoCommentCheckRepo::find_owner(comment_id, user_id).await?;

        // 返回第一个 bool 值，或者默认 false
        Ok(result.first().copied().unwrap_or(false))
    }
}

//////// END
