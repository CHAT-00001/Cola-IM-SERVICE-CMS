// port/src/cola_videoview/manage.rs
// ⏩️ 端口 - ▶ 可乐视频 -  浏览 - 管理
// 2026/8/4 22:11 Created.

////////

use cola_data::market::info::goods::view::GoodsViewInfo;

////////

/// # [MANAGE PORT]
/// * `desc`: `视频浏览管理端口`
#[async_trait::async_trait]
pub trait GoodsViewManagePort: Send + Sync {
    //

    ////////

    /// # [PORT] - 管理员列表
    /// * `desc`: `查看所有的视频浏览记录`
    /// * `condition`: `⚠️ 仅限管理员 / 运营人员`
    async fn admin_get_views_infos(
        &self,
        uid: i64,                // 操作者 ID
        user_id: Option<i64>,    // 用户 ID
        video_id: Option<i64>,   // 视频 ID
        start_time: Option<i64>, // 开始时间
        end_time: Option<i64>,   // 结束时间
        status_code: i16,        // 状态码
        limit: i64,              // 数量
        offset: i64,             // 页码
    ) -> anyhow::Result<(Vec<GoodsViewInfo>), u64>;
}

//////// END
