// port/src/user/vipp/add.rs -- 端口 - USER - 贵宾 - 发布端口
// 2026/8/6 00:35 Created.

////////

use async_trait::async_trait;
use cola_data::cola_user::info::user::UserInfo;
////////

/// # [ADD PORTS]
/// * `desc`: `用户贵宾发布端口`
#[async_trait]
pub trait VipAddPort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 添加
    async fn add_vip(
        &self,
        uid: i64, // UID
        id: i64,  // 目标用户ID
    ) -> anyhow::Result<()>;

    ////////

    /// # 2. [PORT] - 移除
    async fn del_vip(
        &self,
        uid: i64, // UID
        id: i64,  // 目标用户ID
    ) -> anyhow::Result<()>;
}

//////// END
