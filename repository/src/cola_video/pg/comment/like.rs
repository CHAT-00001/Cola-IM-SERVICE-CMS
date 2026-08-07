// repository/src/cola_video/pg/comment/like.rs
// 🗄️ 仓储 - ▶ 可乐视频 - pg - 评论记录 - 点赞
// 2026/8/8 00:48 Created.

////////

use crate::cola_video::pg::comment::comment::CommentRepo;
use anyhow::Error;

use tracing::log;

////////

/// # [LIKE SERVICE] - 发布
/// * `desc`: `▶ 可乐视频 - 👤 视频评论点赞服务`
pub struct CommentLikeService;

// 构造实现
impl CommentLikeService {
    //

    ////////

    /// # 6. [SERVICE] - 点赞
    /// * `desc` 点赞评论
    pub async fn update_comment_like_by_id(
        uid: i64,
        comment_id: i64,
        is_liked: bool,
    ) -> Result<(), anyhow::Error> {
        // 2. 更新点赞状态（幂等）
        CommentRepo::update_comment_like_by_id(uid, comment_id, is_liked).await?;

        Ok(())
    }

    /// # 8. [SERVICE] - 不喜欢
    /// * `desc` 点赞评论
    pub async fn update_comment_unlike_by_id(
        uid: i64,
        comment_id: i64,
        is_unliked: bool,
    ) -> Result<(), anyhow::Error> {
        // 2. 更新点赞状态（幂等）
        CommentRepo::update_comment_unlike_by_id(Some(uid), comment_id, is_unliked).await?;

        Ok(())
    }

    ////////

    /// # 9. [SERVICE] - 检查评论状态
    pub async fn check_comment_state(_uid: i64, comment_id: i64) -> Result<(), anyhow::Error> {
        // TODO: 购买付费视频/电商挂载商品落单逻辑

        Ok(())
    }
}

//////// END
