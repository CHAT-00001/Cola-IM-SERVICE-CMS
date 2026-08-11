// port/src/cola_video/video/manage.rs
// ⏩️ 端口 - VIDEO -  视频 - 管理
// 2026/8/5 00:39 Created.

////////

use cola_data::cola_video::info::video::VideoInfo;

////////

/// # [MANAGE PORTS] - 管理
/// * `desc`: `MARKET - 购物车管理端口`
#[async_trait::async_trait]
pub trait CartManagePort: Send + Sync {
    //

    ////////

    /// # [PORT] - 管理员列表
    /// * `desc`: `管理员查看所有视频`
    /// * `condition`: `⚠️ ADMIN / REVIEWER` - `无视权限/状态`
    async fn admin_get_videos_infos(
        &self,
        uid: i64,                 // 操作者 ID
        user_id: Option<i64>,     // 用户 ID
        video_id: Option<i64>,    // 视频 ID
        category_id: Option<i64>, // 分类 ID
        channel_id: Option<i64>,  // 频道 ID
        keyword: Option<String>,  // 关键词
        start_time: Option<i64>,  // 开始时间
        end_time: Option<i64>,    // 结束时间
        status_code: i16,         // 状态码
        limit: i64,               // 数量
        offset: i64,              // 页码
    ) -> anyhow::Result<(VideoInfo), u64>;
}

//////// END
