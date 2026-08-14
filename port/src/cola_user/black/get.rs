// port/src/user/black/get.rs
// ⏩️ 端口 - 用户 - 黑名单 - 获取
// 2026/6/11 20:13

////////

use async_trait::async_trait;

////////

/// # [ADD PORTS]
/// * `desc`: `USER - 双向黑名单IDs查询端口`
#[async_trait]
pub trait UserBlackGetPort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 用户主动拉黑的
    async fn get_black_ids(
        &self,
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<(Vec<i64>)>;

    ////////

    /// # 2. [PORT] - 用户被动被拉黑的
    async fn get_black_me_ids(
        &self,
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<(Vec<i64>)>;
}

//////// END
