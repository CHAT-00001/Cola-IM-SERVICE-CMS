// repo_adapter/src/cola_video/video/manage.rs
// 🔌 插头 - VIDEO - 视频 - 管理
// 2026/8/6 19:19 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::video::VideoInfo;
use port::cola_video::video::manage::VideoManagePort;

////////

/// # [MANAGE ADAPTER] - 管理
/// * `desc`: `🔌 视频管理适配器`
pub struct VideoManageAdapter;

#[async_trait]
impl VideoManagePort for VideoManageAdapter {
    //

    ////////

    /// # [ADAPTER] - 管理员列表
    async fn admin_get_videos_infos(
        &self,
        uid: i64,
        user_id: Option<i64>,
        video_id: Option<i64>,
        category_id: Option<i64>,
        channel_id: Option<i64>,
        keyword: Option<String>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        status_code: i16,
        limit: i64,
        offset: i64,
    ) -> Result<(VideoInfo), u64> {
        todo!()
    }
}

//////// END
