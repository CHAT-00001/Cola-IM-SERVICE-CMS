// repo_adapter/src/video/share/manage.rs
// 🔌 插头 - 可乐视频 - 分享 - 管理
// 2026/8/6 18:57 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::comment::VideoCommentInfo;
use port::cola_video::share::manage::VideoShareManagePort;

////////

/// # [MANAGE ADAPTER] - share manage
/// * `DESC`: `▶ 视频 - 视频分享记录管理适配器`
#[derive(Debug, Default, Clone)]
pub struct VideoShareManageAdapter;

#[async_trait]
impl VideoShareManagePort for VideoShareManageAdapter {
    async fn admin_get_shares_infos(
        &self,
        uid: i64,                // UID
        user_id: Option<i64>,    // 用户 ID
        video_id: Option<i64>,   // 视频 ID
        start_time: Option<i64>, // 开始时间
        end_time: Option<i64>,   // 结束时间
        status_code: i16,        // 状态码
        limit: i64,              // 数量
        offset: i64,             // 页码
    ) -> Result<(VideoCommentInfo), u64> {
        todo!()
    }
}

//////// END
