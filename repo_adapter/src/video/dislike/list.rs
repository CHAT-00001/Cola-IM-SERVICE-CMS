// repo_adapter/src/video/dislike/list.rs -- 🔌 插头 - VIDEO - 不喜欢 - 列表适配器
// 2026/8/9 22:30 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::dislike::VideoDislikeInfo;
use port::cola_video::dislike::list::VideoDislikeListPort;

////////

/// # [LIST ADAPTER] - dislike list
/// * `desc`: `▶ 视频 - 不喜欢记录列表适配器`
#[derive(Debug, Default, Clone)]
pub struct VideoDislikeListAdapter;

#[async_trait]
impl VideoDislikeListPort for VideoDislikeListAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 用户的
    async fn get_dislike_infos_by_user_id(
        &self,
        uid: i64,
        user_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<VideoDislikeInfo>)> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 视频的
    async fn get_dislike_infos_by_video_id(
        &self,
        uid: i64,
        user_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<VideoDislikeInfo>)> {
        todo!()
    }
}

//////// END
