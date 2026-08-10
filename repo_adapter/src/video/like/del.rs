// repo_adapter/src/cola_video/like/del.rs
// 🔌 适配器 - ▶ 视频 - 点赞记录 - 删除
// 2026/8/6 18:58 Created.

////////

use anyhow::Result;
use async_trait::async_trait;

use port::cola_video::like::del::VideoLikeDelPort;
////////

/// # [DELETE ADAPTER] - like del
/// * `desc`: `▶ 视频 - 点赞记录 Adapter implementation`
#[derive(Debug, Default, Clone)]
pub struct VideoLikeDelAdapter;

#[async_trait]
impl VideoLikeDelPort for VideoLikeDelAdapter {
    async fn single_delete(&self, uid: i64, video_id: i64, like_id: i64) -> Result<(u16)> {
        todo!()
    }

    async fn batch_delete(&self, uid: i64, video_id: i64, like_ids: Vec<i64>) -> Result<(u16)> {
        todo!()
    }

    async fn delete_like_by_video_id(&self, uid: i64, video_id: i64) -> Result<(u16)> {
        todo!()
    }

    async fn delete_like_by_user_id(&self, uid: i64, user_id: i64) -> Result<(u16)> {
        todo!()
    }
}

//////// END
