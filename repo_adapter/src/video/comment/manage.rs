// repo_adapter/src/video/comment/manage.rs
// 🔌 插头 - 可乐视频 - 评论 - 管理
// 2026/8/6 18:55 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::comment::VideoCommentInfo;
use port::cola_video::comment::manage::VideoCommentManagePort;

////////

/// # [ADD ADAPTER] - 发布
/// * `desc`: `可乐视频 - 视频评论管理适配器`
#[derive(Debug, Default, Clone)]
pub struct VideoCommentManageAdapter;

#[async_trait]
impl VideoCommentManagePort for VideoCommentManageAdapter {
    async fn admin_list(
        &self,
        uid: i64,
        user_id: Option<i64>,
        video_id: Option<i64>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        status_code: i16,
        limit: i64,
        offset: i64,
    ) -> Result<(VideoCommentInfo)> {
        todo!()
    }
}
