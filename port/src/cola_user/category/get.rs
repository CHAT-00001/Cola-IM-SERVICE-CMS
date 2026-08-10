// port/src/cola_user/category/get.rs
// 用户 - port - 关注 - 获取
// 2026/8/5 21:56 Created.

////////

use async_trait::async_trait;

////////

/// # [ADD PORTS]
/// * `desc`: `用户关注发布端口`
#[async_trait]
pub trait UserCategoryGetPort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 我的
    /// * `desc`: `获取我关注的用户IDs`
    async fn get_my_follow_ids(
        &self,
        uid: i64,    // UID
        id: i64,     // 目标用户ID
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<i64>)>;

    ////////

    /// # 2. [PORT] - 她的
    /// * `desc`: `获取TA关注的用户IDs`
    async fn get_he_follow_ids(
        &self,
        uid: i64,    // UID
        id: i64,     // 目标用户ID
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<i64>)>;

    ////////

    /// # 3. [PORT] - 关注我的
    /// * `desc`: `获取关注我的用户IDs`
    async fn get_follow_me_ids(
        &self,
        uid: i64,    // UID
        id: i64,     // 目标用户ID
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<i64>)>;

    ////////

    /// # 4. [PORT] - 关注她的
    /// * `desc`: `获取关注TA的用户IDs`
    async fn get_follow_he_ids(
        &self,
        uid: i64,    // UID
        id: i64,     // 目录用户ID
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<i64>)>;
}

//////// END