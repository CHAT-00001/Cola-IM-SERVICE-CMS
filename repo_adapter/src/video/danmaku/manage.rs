// repo_adapter/src/cola_video/danmaku/manage.rs
// 🔌 插头 - VIDEO - 弹幕 - 管理
// 2026/8/6 18:56 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::comment::VideoCommentInfo;
use port::cola_video::danmaku::manage::VideoDanmakuManagePort;

////////

/// # [ADAPTER] - danmaku manage
#[derive(Debug, Default, Clone)]
pub struct VideoDanmakuManageAdapter;

#[async_trait]
impl VideoDanmakuManagePort for VideoDanmakuManageAdapter {

    /// # [ADAPTER] - 管理员列表
    async fn admin_get_danmakus_infos(
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

//////// END
