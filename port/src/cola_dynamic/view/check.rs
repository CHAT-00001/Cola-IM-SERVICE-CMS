// /check.rs
// ⏩️ 端口 - ⏹ 可乐动态 - 浏览 - 检查
// 2026/8/10 04:50 Created.

////////

////////

use cola_data::cola_video::info::comment::VideoCommentInfo;

////////

/// # [CHECK PORTS]
/// * `desc`: `⏹ 动态 - 动态浏览获取端口`
#[async_trait::async_trait]
pub trait DynamicViewCheckPort: Send + Sync {
    //

    ////////

    /// 1. [PORT] - 健康
    async fn check_health(
        &self,
        profile_id: i64, // 资料 ID
    ) -> anyhow::Result<(bool)>;

    ////////

    /// 2. [PORT] - 是否看过
    async fn is_visited(
        &self,
        user_id: i64,    // 用户 ID
        profile_id: i64, // 主页 ID
    ) -> anyhow::Result<(bool)>;
}

//////// END
