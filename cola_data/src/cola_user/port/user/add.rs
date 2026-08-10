// cola_user/port/cola_user/add.rs
// 用户 - port - 用户 - 发布
// 2026/8/5 22:02 Created.

////////

use crate::cola_user::command::user::add::UserCommand;
use crate::cola_user::command::user::update::UpdateUserCommand;
use crate::cola_user::info::user::UserInfo;

////////

/// # [ADD PORTS] - 发布
/// * `desc`: `用户发布端口`
#[async_trait::async_trait]
pub trait UserAddPort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 保存
    async fn save_user(
        &self,
        cmd: UserCommand, // 命令
    ) -> anyhow::Result<(UserInfo)>;

    ////////

    /// # 2. [PORT] - 更新
    async fn update_user(
        &self,
        cmd: UpdateUserCommand, // 命令
    ) -> anyhow::Result<(UserInfo)>;

    ////////

    /// # 3. [PORT] - 更新头像
    async fn update_avatar(
        &self,
        uid: i64,      // 操作者ID
        media_id: i64, // 媒体ID
    ) -> anyhow::Result<()>;

    ////////

    /// # 4. [PORT] - 更新背景图
    async fn update_bg(
        &self,
        uid: i64,      // 操作者ID
        media_id: i64, // 媒体ID
    ) -> anyhow::Result<()>;
}

//////// END
