// cola_dynamic/port/hotlist/manage.rs
// ⏩️ 端口 - ⏹ 可乐动态 - port - 推荐 - 管理
// 2026/8/5 00:01 Created.

////////⏩

use cola_data::cola_video::info::comment::VideoCommentInfo;

////////

/// # [MANAGE PORTS] - 管理
/// * `desc`: `动态推荐管理端口`
#[async_trait::async_trait]
pub trait DynamicRecommendManagePort: Send + Sync {
    //

    ////////

    /// # [PORT] - 管理员列表
    /// * `desc`: `管理员查看所有推荐记录列表`
    /// * `condition`: `⚠️ ADMIN` - `仅限管理员`
    async fn admin_get_recommend_infos(
        &self,
        uid: i64,
        user_id: Option<i64>,    // 用户 ID
        dynamic_id: Option<i64>, // 动态 ID
        start_time: Option<i64>, // 开始时间
        end_time: Option<i64>,   // 结束时间
        status_code: i16,        // 状态码
        limit: i64,              // 数量
        offset: i64,             // 页码
    ) -> anyhow::Result<(VideoCommentInfo)>;
}

//////// END
