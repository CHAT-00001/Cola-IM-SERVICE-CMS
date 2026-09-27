// repo_adapter/src/video/comment/get.rs -- ADAPTER - VIDEO - 评论 - 获取适配器
// 2026/8/6 18:55 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::comment::CommentInfo;
use port::cola_video::comment::get::VideoCommentGetPort;
use service::cola_video::comment::get::CommentGetService;

////////

/// # [GET ADAPTER] - 视频评获取适配器
/// * `desc`: `COLA VIDEO - Comment Get Adapter.`
#[derive(Debug, Default, Clone)]
pub struct VideoCommentGetAdapter;

#[async_trait]
impl VideoCommentGetPort for VideoCommentGetAdapter {
    //

    ////////

    /// # 1. [ADAPTER SERVICE] - 用户的评论信息
    async fn get_comment_by_user_id(
        &self,
        user_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<CommentInfo>)> {
        ////////

        // 👤 - 服务
        CommentGetService::get_comments_by_user_id(user_id, offset, limit).await
    }

    ////////

    /// # 2. [ADAPTER SERVICE] - 视频的评论信息
    async fn get_comment_by_video_id(
        &self,
        video_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<CommentInfo>)> {
        ////////

        // 👤 - 服务
        CommentGetService::get_comments_by_video_id(video_id, offset, limit).await
    }
}

//////// END