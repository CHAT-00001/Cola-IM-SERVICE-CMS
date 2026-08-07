// repo_adapter/src/user/add/mod.rs
// 适配器 - USER - Add (原子化解耦)
// 2026-08-12
// 2026/8/6 对接新版 UserAddPort + 原子化子模块

////////

use async_trait::async_trait;
use cola_data::user::command::new::UserCommand;
use cola_data::user::command::user::update::UpdateUserCommand;
use cola_data::user::info::user::UserInfo;
use cola_data::user::port::user::add::UserAddPort;

pub mod add;
pub mod del;
pub mod update;

////////

/// # [ADAPTER] - 用户 Add 端口适配器
pub struct UserAddPortAdapter;

#[async_trait]
impl UserAddPort for UserAddPortAdapter {

    ////////

    /// # 1. [ADAPTER] - 保存用户
    async fn save_user(
        &self,
        cmd: UserCommand, // 命令
    ) -> anyhow::Result<UserInfo> {
        add::save_user(cmd).await
    }

    ////////

    /// # 2. [ADAPTER] - 更新用户资料
    async fn update_user(
        &self,
        cmd: UpdateUserCommand, // 命令
    ) -> anyhow::Result<UserInfo> {
        update::update_user(cmd).await
    }

    ////////

    /// # 3. [ADAPTER] - 更新头像
    async fn update_avatar(
        &self,
        uid: i64, // 操作者ID
        media_id: i64, // 媒体ID
    ) -> anyhow::Result<()> {
        update::update_avatar(uid, media_id).await
    }

    ////////

    /// # 4. [ADAPTER] - 更新背景图
    async fn update_bg(
        &self,
        uid: i64, // 操作者ID
        media_id: i64, // 媒体ID
    ) -> anyhow::Result<()> {
        update::update_bg(uid, media_id).await
    }
}

//////// END