// port/src/user/ban/add.rs
// 用户 - port - 封禁 - 发布
// 2026/8/5 21:33 Created.

////////

use async_trait::async_trait;

////////

/// # [ADD PORTS] - 发布
/// * `desc`: `用户封禁发布端口`
#[async_trait]
pub trait UserBanAddPort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 添加
    /// * `desc`: `发布封禁用户`
    async fn save_banned(
        &self,
        uid: i64, // UID
        id: i64,  // 目标用户ID
    ) -> anyhow::Result<()>;

    ////////

    /// # 2. [PORT] - 解除封禁
    /// * `desc`: `修改封禁记录实现解封`
    async fn remove_banned(
        &self,
        uid: i64, // UID
        id: i64,  // 目标用户ID
    ) -> anyhow::Result<()>;
}

//////// END
