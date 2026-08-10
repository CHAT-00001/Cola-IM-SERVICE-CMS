// port/src/cola_video/danmaku/manage.rs
// ⏩️ 端口 - ▶ 可乐视频 - 弹幕 - 管理
// 2026/8/5 00:06 Created.

////////

use cola_data::cola_video::info::comment::VideoCommentInfo;
use cola_data::cola_video::info::danmaku::DanmakuInfo;

////////

/// # [ADD SERVICE] - 管理
/// * `desc`: `弹幕管理服务`
#[async_trait::async_trait]
pub trait VideoDanmakuManagePort: Send + Sync {
    //

    ////////

    /// # [PORT] - 管理员列表
    /// * `desc`: `🗣 ADMIN` - `仅限管理员 / 运营人员`
    /// * `condition`: `⚠️ WARNING`
    async fn admin_get_danmakus_infos(
        &self,
        uid: i64,                // 操作者 ID
        user_id: Option<i64>,    // 用户 ID
        video_id: Option<i64>,   // 视频 ID
        start_time: Option<i64>, // 开始时间
        end_time: Option<i64>,   // 结束时间
        status_code: i16,        // 状态码
        limit: i64,              // 数量
        offset: i64,             // 页码
    ) -> anyhow::Result<(VideoCommentInfo)>;
}

//////// END
