// repo_adapter/src/cola_video/hotlist/manage.rs
// 🔌 插头 - 可乐视频 - 上热门 - 管理
// 2026/8/6 19:04 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::comment::VideoCommentInfo;
use port::cola_video::hotlist::manage::VideoHotlistManagePort;

////////

/// # [ADAPTER] - hotlist manage
/// * `desc`: `▶ 视频 - 上热门记录管理适配器`
#[derive(Debug, Default, Clone)]
pub struct VideoHotlistManageAdapter;

#[async_trait]
impl VideoHotlistManagePort for VideoHotlistManageAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 管理员列表
    async fn admin_get_recommends_infos(
        &self,
        uid: i64,                // UID
        user_id: Option<i64>,    // 用户 ID
        video_id: Option<i64>,   // 视频 ID
        start_time: Option<i64>, // 开始时间
        end_time: Option<i64>,   // 结束时间
        status_code: i16,        // 状态码
        limit: i64,              // 数量
        offset: i64,             // 页码
    ) -> Result<(VideoCommentInfo)> {
        todo!()
    }
}

//////// END
