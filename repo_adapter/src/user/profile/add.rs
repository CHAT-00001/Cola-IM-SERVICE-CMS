// repo_adapter/src/user/user/add.rs -- 适配器层 - USER - 用户资料 - 发布适配器
// 2026/8/6 04:18 Created.

////////

use async_trait::async_trait;
use cola_data::cola_user::command::user::add::UserCommand;
use cola_data::cola_user::command::user::update::UpdateUserCommand;
use cola_data::cola_user::info::user::UserInfo;
use port::cola_user::profile::add::UserAddPort;
use repository::user::pg::user::add::UserAddRepo;

////////

/// # [ADD ADAPTER] - 发布
/// * `desc`: `用户发布服务`
pub struct UserAddAdapter;

// 构造实现
#[async_trait]
impl UserAddPort for UserAddAdapter {
    //

    ////////

    /// # 1. [SERVICE] - 保存
    /// * `desc`: `保存新用户记录`
    async fn save_user(
        &self,
        cmd: UserCommand, // 命令
    ) -> anyhow::Result<(UserInfo)> {
        // 1. Call ..
        let entity = cmd.new();

        // 2. Call ..
        let saved = UserAddRepo::save_user(entity)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 USER ADD ADAPTER]: ❌️ 保存用户失败: {}", e))?;

        Ok(saved.into())
    }

    async fn update_user(&self, cmd: UpdateUserCommand) -> anyhow::Result<(UserInfo)> {
        let entity = cmd.to_entity(0);
        let saved = UserAddRepo::save_user(entity)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 USER UPDATE ADAPTER]: ❌️ 更新用户失败: {}", e))?;

        Ok(saved.into())
    }

    async fn update_avatar(&self, uid: i64, media_id: i64) -> anyhow::Result<()> {
        todo!()
    }

    async fn update_bg(&self, uid: i64, media_id: i64) -> anyhow::Result<()> {
        todo!()
    }
}

//////// END
