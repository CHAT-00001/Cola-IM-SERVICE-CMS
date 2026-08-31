// port/src/user/vip/check.rs
// ⏩️ 端口 - 🗣 可乐用户 - 贵宾 - 检查
// 2026/8/6 00:35 Created.

////////

use async_trait::async_trait;

////////

/// # [CHECK PORTS]
/// * `desc`: `贵宾检查端口`
#[async_trait]
pub trait VipCheckPort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 检查
    /// * `desc`: `检查是否已经开通贵宾`
    async fn is_viper(
        &self,
        uid: i64, // 操作者ID
        id: i64,  // 目标用户ID
    ) -> anyhow::Result<(bool)>;
    async fn is_vip(&self, uid: i64, user_id: i64) -> anyhow::Result<bool>;
}

//////// END
