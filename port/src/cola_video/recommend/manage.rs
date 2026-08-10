// cola_video/recommend/manage.rs
// ⏩️ 端口 - ▶ 可乐视频 - 推荐 - 管理
// 2026/8/5 00:01 Created.

////////

use cola_data::cola_video::info::recommend::VideoRecommendInfo;

////////

/// # [MANAGE PORTS] - 管理
/// * `desc`: `视频收藏管理端口`
#[async_trait::async_trait]
pub trait VideoRecommendManagePort: Send + Sync {
    //

    ////////

    /// # [PORT] - 管理员列表
    /// * `desc`: `🗣 ADMIN` - `仅限管理员 / 运营人员`
    /// * `condition`: `⚠️ WARNING`
    async fn admin_get_recommends_infos(
        &self,
        uid: i64,                // 操作者 ID
        user_id: Option<i64>,    // 用户 ID
        video_id: Option<i64>,   // 视频 ID
        start_time: Option<i64>, // 开始时间
        end_time: Option<i64>,   // 结束时间
        status_code: i16,        // 状态码
        limit: i64,              // 数量
        offset: i64,             // 页码
    ) -> anyhow::Result<(Vec<VideoRecommendInfo>, u64)>;
}

//////// END