// servicey/src/cola_video/comment/get.rs
// 👤 服务 - ▶ 可乐视频  - 评论 - 获取
// 2026/8/2 17:15 Created.

////////

use anyhow::Error;
use cola_data::cola_video::info::comment::VideoCommentInfo;
use repository::cola_video::pg::comment::comment::CommentRepo;
use tracing::log;

////////

/// # [SERVICE] - 视频 评论 获取 服务
pub struct CommentGetService;

impl CommentGetService {
    //

    ////////

    /// # 1. [SERVICE] - 获取视频的评论
    /// * `video_id`  视频 ID
    pub async fn get_comments_by_video_id(
        video_id: i64,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<VideoCommentInfo>, anyhow::Error> {
        let entities = CommentRepo::find_new_comments_by_video_id(video_id, offset, limit).await?;

        // handler -> info
        let infos: Vec<VideoCommentInfo> = entities
            .into_iter()
            .map(VideoCommentInfo::from_entity)
            .collect();

        Ok(infos)
    }

    ////////

    /// # 2. [SERVICE] - 查找用户的评论
    /// * `user_id`  用户 ID
    pub async fn get_comments_by_user_id(
        video_id: i64,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<VideoCommentInfo>, anyhow::Error> {
        let entities = CommentRepo::find_comments_by_user_id(video_id, offset, limit).await?;

        // handler -> info
        let infos: Vec<VideoCommentInfo> = entities
            .into_iter()
            .map(VideoCommentInfo::from_entity)
            .collect();

        Ok(infos)
    }

    ////////

    /// # 3. [SERVICE] - 浏览我发布评论
    /// * `user_id`  视频 ID
    pub async fn get_my_publish_comments(
        user_id: i64,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<VideoCommentInfo>, anyhow::Error> {
        let entities = CommentRepo::find_comments_by_user_id(user_id, offset, limit).await?;

        // handler -> info
        let infos: Vec<VideoCommentInfo> = entities
            .into_iter()
            .map(VideoCommentInfo::from_entity)
            .collect();

        Ok(infos)
    }
}

//////// END
