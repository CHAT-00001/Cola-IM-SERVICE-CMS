// port/src/user/sharek/manage.rs
// ⏩️ 端口 - 🗣 用户 - 分享 - 管理
// 2026/8/5 21:35 Created.

////////

use async_trait::async_trait;
use cola_data::cola_user::info::config::UserConfigInfo;
////////

/// # [MANAGE PORTS]
/// * `desc`: `🗣 用户 - 用户分享记录管理端口`
#[async_trait]
pub trait UserShareManagePort: Send + Sync + 'static {
    ////////

    /// # 1. [PORT] - 配置
    async fn get_config(&self, user_id: i64) -> anyhow::Result<(UserConfigInfo)>;

    ////////
}

//////// END
