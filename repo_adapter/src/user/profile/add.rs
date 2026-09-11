// repo_adapter/src/user/user/add.rs -- 适配器层 - USER - 用户资料 - 发布适配器
// 2026/8/6 04:18 Created.

////////

use async_trait::async_trait;
use cola_data::cola_user::command::user::add::UserCommand;
use cola_data::cola_user::command::user::update::UpdateUserCommand;
use cola_data::cola_user::info::user::UserInfo;
use port::cola_user::profile::add::UserAddPort;
use repository::user::pg::user::add::UserAddRepo;
use repository::user::redis::profile::UserProfileCache;
use tracing::warn;

////////

/// # [ADD ADAPTER] - 发布
/// * `desc`: `用户发布服务`
pub struct UserAddAdapter;

// 构造实现
#[async_trait]
impl UserAddPort for UserAddAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 创建用户
    /// * `desc`: `PG 创建成功后回填用户资料缓存，缓存失败不影响主流程`
    async fn create_user(
        &self,
        cmd: UserCommand, // 命令
    ) -> anyhow::Result<(UserInfo)> {
        let entity = cmd.new();

        let saved = UserAddRepo::create_user(entity)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 USER ADD ADAPTER]: ❌️ 创建用户失败: {}", e))?;
        let user_info: UserInfo = saved.into();

        if let Err(error) = UserProfileCache::set_user_info(&user_info).await {
            warn!(
                "[🤐 USER ADD ADAPTER] - ❌️ 新用户资料缓存回填失败: user_id={}, error={}",
                user_info.id, error
            );
        }

        Ok(user_info)
    }

    async fn update_user(&self, cmd: UpdateUserCommand) -> anyhow::Result<(UserInfo)> {
        let entity = cmd.to_entity(0);
        let saved = UserAddRepo::create_user(entity)
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
