// repo_adapter/src/video/comment/list.rs -- ADAPTER - VIDEO - 评论 - 列表
// 2026/8/6 18:55 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::comment::CommentInfo;
use port::cola_video::comment::list::VideoCommentListPort;

////////

/// # [LIST SERVICE] - 视频评论列表适配器
/// * `desc`: `COLA VIDEO - Comment List Service Adapter.`
#[derive(Debug, Default, Clone)]
pub struct VideoCommentListAdapter;

#[async_trait]
impl VideoCommentListPort for VideoCommentListAdapter {
    async fn get_my_like_record(
        &self,
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(CommentInfo)> {
        todo!()
    }

    async fn get_he_like_record(
        &self,
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(CommentInfo)> {
        todo!()
    }
}

//////// END