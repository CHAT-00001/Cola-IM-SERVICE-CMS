// port/src/user/black/add.rs
// ⏩️ 端口 - 🗣 可乐用户 - 黑名单 - 发布
// 2026/8/5 21:33 Created.

////////

use async_trait::async_trait;

////////

/// # [ADD PORTS]
/// * `desc`: `用户黑名单发布端口`
#[async_trait]
pub trait UserBlackAddPort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 添加
    async fn add_black(
        &self,
        uid: i64, // UID
        id: i64,  // 目标用户ID
    ) -> anyhow::Result<()>;

    ////////

    /// # 2. [PORT] - 移除
    async fn del_black(
        &self,
        uid: i64, // UID
        id: i64,  // 目标用户ID
    ) -> anyhow::Result<()>;
}

//////// END
