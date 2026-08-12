// repo_adapter/src/cola_video/like/list.rs
// 🔌 适配器 - VIDEO - 点赞 - 列表
// 2026/8/6 18:57 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::comment::VideoCommentInfo;
use port::cola_video::like::list::VideoLikeListPort;

////////

/// # [ADAPTER] - like get
/// * `desc`: Adapter implementation
#[derive(Debug, Default, Clone)]
pub struct VideoLikeListAdapter;

#[async_trait]
impl VideoLikeListPort for VideoLikeListAdapter {
    async fn get_like_infos_by_user_id(
        &self,
        uid: i64,
        user_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(VideoCommentInfo)> {
        todo!()
    }

    async fn get_like_infos_by_video_id(
        &self,
        uid: i64,
        user_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(VideoCommentInfo)> {
        todo!()
    }
}

//////// END
