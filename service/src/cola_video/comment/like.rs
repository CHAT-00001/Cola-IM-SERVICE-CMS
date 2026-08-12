// servicey/src/video/comment/like.rs
// 服务 - ▶ VIDEO  - 评论 - 点赞/不喜欢
// 2026/8/2 17:15 Created.

////////

use anyhow::Error;
use cola_data::cola_video::command::comment::CommentCommand;
use cola_data::cola_video::entity::video::video::VideoEntity;
use cola_data::cola_video::info::comment::VideoCommentInfo;
use tracing::log;
use repository::cola_video::pg::comment::comment::CommentRepo;
use repository::cola_video::pg::video::home::VideoRepo;
////////

/// # [SERVICE] - 评论 互动
pub struct CommentLikeService;

impl CommentLikeService {
    //

    ////////

    /// # 1. [SERVICE] - 点赞
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
    /// * `desc` 不喜欢评论
    pub async fn update_comment_unlike_by_id(
        uid: i64,
        comment_id: i64,
        is_disliked: bool,
    ) -> Result<(), anyhow::Error> {
        // 2. 更新点赞状态（幂等）
        CommentRepo::update_comment_unlike_by_id(Some(uid), comment_id, is_disliked).await?;

        Ok(())
    }
}

//////// END
