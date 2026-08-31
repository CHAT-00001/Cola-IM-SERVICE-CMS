// repo_adapter/src/video/comment/get.rs
// 🔌 插头 - 可乐视频 - 评论 - 获取IDs
// 2026/8/6 18:55 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::comment::VideoCommentInfo;
use port::cola_video::comment::get::VideoCommentGetPort;

////////

/// # [GET ADAPTER] - 发布
/// * `desc`: `VIDEO - 视频评论获取适配器`
#[derive(Debug, Default, Clone)]
pub struct VideoCommentGetAdapter;

#[async_trait]
impl VideoCommentGetPort for VideoCommentGetAdapter {
    async fn get_comment_by_user_id(
        &self,
        user_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<VideoCommentInfo>)> {
        todo!()
    }

    async fn get_comment_by_video(
        &self,
        video_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<VideoCommentInfo>)> {
        todo!()
    }
}
