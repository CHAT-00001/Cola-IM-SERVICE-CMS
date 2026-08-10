// port/src/cola_dynamic/view/manage.rs
// ⏩️ 端口 - ⏹ 可乐动态 - 浏览 - 管理
// 2026/8/4 22:11 Created.

////////

use cola_data::cola_video::info::comment::VideoCommentInfo;

////////

/// # [VIDEO VIEW SERVICE PORT]
/// * `desc`: `视频浏览管理端口`
#[async_trait::async_trait]
pub trait DynamicViewManagePort: Send + Sync {
    //

    ////////

    /// # [PORT] - 全部浏览记录
    /// * `desc`: `⏹ 可乐动态` - `浏览记录`
    /// * `condition`: `⚠️ ADMIN` - `仅限管理员`
    async fn admin_get_hotlist_infos(
        &self,
        uid: i64,                 // UID
        dynamic_id: i64,          // 动态 ID
        status_code: Option<i16>, // 状态码
        is_deleted: Option<bool>, // 是否删除
        limit: i64,               // 数量
        offset: i64,              // 页码
    ) -> anyhow::Result<(Vec<VideoCommentInfo>)>;
}

//////// END
