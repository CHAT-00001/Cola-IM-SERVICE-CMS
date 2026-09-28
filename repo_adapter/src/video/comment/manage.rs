// repo_adapter/src/video/comment/manage.rs -- ADAPTER - VIDEO - 评论 - 管理
// 2026/8/6 18:55 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::comment::CommentInfo;
use port::cola_video::comment::manage::VideoCommentManagePort;

////////

/// # [MANAGE ADAPTER] - 视频评论管理适配器
/// * `desc`: `COLA VIDEO - Comment Manage Service Adapter.`
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
    ) -> Result<(Vec<CommentInfo>, u64)> {
        todo!()
    }
}

//////// END
