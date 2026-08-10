// repo_adapter/src/video/collect/list.rs
// 🔌 适配器  - ▶ 视频 - 收藏 - 列表
// 2026/8/9 20:24 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::comment::VideoCommentInfo;
use port::cola_video::collect::list::VideoCollectListPort;

////////

/// # [LIST ADAPTER] - 列表
/// * `desc`: `▶ 视频 - 收藏记录记录`
#[derive(Debug, Default, Clone)]
pub struct VideoCollectListAdapter;

#[async_trait]
impl VideoCollectListPort for VideoCollectListAdapter {
    async fn get_my_like_record(
        &self,
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(VideoCommentInfo)> {
        todo!()
    }

    async fn get_he_like_record(
        &self,
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(VideoCommentInfo)> {
        todo!()
    }

    async fn del_comment_record(&self, comment_id: i64) -> Result<()> {
        todo!()
    }

    async fn del_comments_record(&self, comment_ids: Vec<i64>) -> Result<()> {
        todo!()
    }
}

//////// END
