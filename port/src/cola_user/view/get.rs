// port/src/user/view/get.rs -- 端口 - USER - 浏览 - 获取端口
// 2026/8/6 00:50 Created.

////////

use async_trait::async_trait;

////////

/// # [GET PORTS]
/// * `desc`: `USER - 用户浏览获取端口`
#[async_trait]
pub trait UserViewGetPort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 我看过谁
    /// * `desc`: `获取我拉黑的用户IDs`
    async fn get_views_ids(
        &self,
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<(Vec<i64>)>;

    ////////

    /// # 2. [PORT] - 谁看过我
    /// * `desc`: `获取TA拉黑的用户IDs`
    async fn get_view_me_ids(
        &self,
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<(Vec<i64>)>;
}

//////// END
