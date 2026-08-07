// servicey/src/cola_video/comment/like.rs
// 👤 服务 - ▶ 可乐视频  - 评论 - 点赞
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

/// # [SERVICE] - 视频 评论 点赞 服务
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

    ////////

    /// # 10. [SERVICE] - 查找最新的视频列表
    pub async fn find_new_video_list(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoEntity>, anyhow::Error> {
        VideoRepo::find_new_list(limit, offset)
            .await
            .map_err(|e| anyhow::anyhow!("[🔌 ADAPTER]: ▶ 获取最新视频列表失败: {}", e))
    }

    ////////

    /// # 11. [SERVICE] - 查找热门的视频列表
    pub async fn find_hot_video_list(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoEntity>, anyhow::Error> {
        VideoRepo::find_hot_list(limit, offset)
            .await
            .map_err(|e| anyhow::anyhow!("[🔌 ADAPTER]: ▶ 获取热门视频列表失败: {}", e))
    }

    ////////

    /// # 12. [SERVICE] - 查找推荐的视频列表
    pub async fn find_recommend_video_list(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoEntity>, anyhow::Error> {
        VideoRepo::find_recommend_list(limit, offset)
            .await
            .map_err(|e| anyhow::anyhow!("[🔌 ADAPTER]: ▶ 获取推荐视频列表失败: {}", e))
    }

    ////////
}

//////// END
