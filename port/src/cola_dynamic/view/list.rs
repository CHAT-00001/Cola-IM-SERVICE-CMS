// /list.rs
// ⏩️ 端口 - ⏹ 可乐动态 - 浏览 - 列表
// 2026/8/10 04:49 Created.

////////

use cola_data::cola_video::info::comment::VideoCommentInfo;

////////

/// # [DYNAMIC VIEW PORTS]
/// * `desc`: `⏹ 可乐动态 - 动态浏览获取端口`
#[async_trait::async_trait]
pub trait DynamicViewListPort: Send + Sync {
    //

    ////////

    /// 1. [PORT] - 用户的
    async fn get_view_infos_by_user_id(
        &self,
        user_id: i64, // 用户 ID
    ) -> anyhow::Result<(Vec<VideoCommentInfo>)>;

    ////////

    /// 2. [PORT] - 资料的
    async fn get_view_infos_by_profile_id(
        &self,
        user_id: i64, // 用户 ID
    ) -> anyhow::Result<(Vec<VideoCommentInfo>)>;
}

//////// END
