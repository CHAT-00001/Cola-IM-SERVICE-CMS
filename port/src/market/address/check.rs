// port/src/market/address/check.rs
// ⏩️ 端口 - MARKET - 地址簿 - 检查
// 2026/8/11 23:32 Created.

////////


////////

/// # [CHECK PORTS] - 检查端口
/// * `desc`: `MARKET - 地址状态与权限检查接口`
#[async_trait::async_trait]
pub trait AddressCheckPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 检查地址状态
    /// * `desc`: 返回地址的状态码
    async fn check_status(
        &self,
        address_id: i64, // 地址 ID
    ) -> anyhow::Result<i16>;

    ////////

    /// # 2. [PORT] - 检查所有权
    /// * `desc`: 验证该地址是否属于指定用户
    async fn is_owner(
        &self,
        user_id: i64,    // 用户 ID
        address_id: i64, // 地址 ID
    ) -> anyhow::Result<bool>;
}

//////// END