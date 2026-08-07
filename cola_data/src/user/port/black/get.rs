// user/port/black/get.rs
// 用户 - port - 黑名单 - 获取
// 2026/6/11 20:13

////////

use async_trait::async_trait;

////////

/// # [ADD PORTS]
/// * `desc`: `用户黑名单发布端口`
#[async_trait]
pub trait BlackGetPort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 我的
    /// * `desc`: `获取我拉黑的用户IDs`
    async fn get_my_black_ids(
        &self,
        uid: i64,    // UID
        id: i64,     // 目标用户ID
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<i64>)>;

    ////////

    /// # 2. [PORT] - 她的
    /// * `desc`: `获取TA拉黑的用户IDs`
    async fn get_he_black_ids(
        &self,
        uid: i64,    // UID
        id: i64,     // 目标用户ID
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<i64>)>;

    ////////

    /// # 3. [PORT] - 拉黑我的
    /// * `desc`: `获取拉黑我的用户IDs`
    async fn get_black_me_ids(
        &self,
        uid: i64,    // UID
        id: i64,     // 目标用户ID
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<i64>)>;

    ////////

    /// # 4. [PORT] - 拉黑她的
    /// * `desc`: `获取拉黑TA的用户IDs`
    async fn get_black_he_ids(
        &self,
        uid: i64,    // UID
        id: i64,     // 目录用户ID
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<i64>)>;
}

//////// END