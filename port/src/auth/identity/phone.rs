// port/src/auth/identity/phone.rs
// ✅ PORT - AUTH 手机身份绑定
// 2026/8/20 Created.

////////

/// # [PORT] - 手机身份查询与绑定
/// * `desc`: `登录时检查手机号是否绑定用户，并为新用户绑定手机号`
#[async_trait::async_trait]
pub trait PhoneIdentityPort: Send + Sync {
    /// # 1. [PORT] - 查询手机号绑定用户
    /// * `desc`: `返回已绑定的用户ID；未命中返回 None`
    async fn find_user_id_by_phone(&self, phone: &str) -> anyhow::Result<Option<i64>>;

    /// # 2. [PORT] - 绑定手机号
    /// * `desc`: `将标准化手机号绑定到用户身份表`
    async fn bind_phone(&self, user_id: i64, phone: &str) -> anyhow::Result<()>;
}

//////// END
