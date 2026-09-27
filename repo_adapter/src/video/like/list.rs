// repo_adapter/src/cola_video/like/list.rs -- ADAPTER - VIDEO - 点赞 - 列表
// 2026/8/6 18:57 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::comment::CommentInfo;
use port::cola_video::like::list::VideoLikeListPort;

////////

/// # [ADAPTER] - 视频内容点赞列表
/// * `desc`: `COLA VIDEO - Content Like List Adapter implementation`
#[derive(Debug, Default, Clone)]
pub struct VideoLikeListAdapter;

#[async_trait]
impl VideoLikeListPort for VideoLikeListAdapter {
    //

    ///////

    /// # 1. [SERVICE ADAPTER] - 用户点赞的记录信息
    async fn get_like_infos_by_user_id(
        &self,
        uid: i64,
        user_id: i64, // 用户 ID
        limit: i64,
        offset: i64,
    ) -> Result<(CommentInfo)> {
        todo!()
    }

    ///////

    /// # 2. [SERVICE ADAPTER] - 视频被点赞的记录信息
    async fn get_like_infos_by_video_id(
        &self,
        uid: i64,
        video_id: i64, // 视频 ID
        limit: i64,
        offset: i64,
    ) -> Result<(CommentInfo)> {
        todo!()
    }
}

//////// END
