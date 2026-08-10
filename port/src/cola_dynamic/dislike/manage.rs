// port/src/cola_dynamic/dislike/manage.rs
// ⏩️ 端口 - 可乐动态 - 不喜欢 - 管理
// 2026/8/5 16:00 Created.

////////

use cola_data::cola_video::info::comment::VideoCommentInfo;

////////

/// # [MANAGE PORTS] - 管理
/// * `desc`: `动态不喜欢管理端口`
#[async_trait::async_trait]
pub trait DislikeManagePort: Send + Sync {
    //

    ////////

    /// # [PORT] -
    async fn admin_get_dislike_infos(
        &self,
        uid: i64,
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(VideoCommentInfo)>;
}

//////// END
