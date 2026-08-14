// port/src/user/share/add.rs
// ⏩️ 端口 - 🗣 用户 - 分享 - 发布
// 2026/8/5 21:33 Created.

////////

use async_trait::async_trait;
use cola_data::cola_user::info::user::UserInfo;

////////

/// # [ADD PORTS]
/// * `desc`: `🗣 用户 - 用户主页分享发布端口`
#[async_trait]
pub trait UserShareAddPort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 添加
    async fn add_share(
        &self,
        uid: i64, // UID
        id: i64,  // 目标用户ID
    ) -> anyhow::Result<()>;

    ////////

    /// # 2. [PORT] - 移除
    async fn del_share(
        &self,
        uid: i64, // UID
        id: i64,  // 目标用户ID
    ) -> anyhow::Result<()>;
}

//////// END
