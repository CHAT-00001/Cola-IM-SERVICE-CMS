// repo_adapter/src/user/profile/history.rs -- 用户资料历史适配器
// 2026/9/23 Created.

////////

use async_trait::async_trait;
use cola_data::app::page::PageInfo;
use cola_data::cola_user::command::user::update::UpdateUserCommand;
use cola_data::cola_user::info::user::UserInfo;
use cola_data::cola_user::info::user_history::UserHistoryInfo;
use port::cola_user::profile::history::UserHistoryPort;
use repository::user::pg::profile::history::UserHistoryRepo;
use repository::user::pg::user::get::UserGetRepo;
use repository::user::redis::profile::UserProfileCache;
use tracing::warn;

////////

/// # [ADAPTER] - 用户资料历史适配器
pub struct UserHistoryAdapter;

#[async_trait]
impl UserHistoryPort for UserHistoryAdapter {
    async fn update_profile(&self, uid: i64, cmd: UpdateUserCommand) -> anyhow::Result<UserInfo> {
        UserHistoryRepo::update_profile(uid, cmd).await?;
        Self::load_updated_user(uid).await
    }

    async fn list(
        &self,
        uid: i64,
        is_avatar: bool,
        page: i64,
        qty: i64,
    ) -> anyhow::Result<(Vec<UserHistoryInfo>, PageInfo)> {
        let (rows, page) = UserHistoryRepo::list(uid, is_avatar, page, qty).await?;
        Ok((rows.into_iter().map(Into::into).collect(), page))
    }

    async fn activate(&self, uid: i64, is_avatar: bool, id: i64) -> anyhow::Result<UserInfo> {
        UserHistoryRepo::activate(uid, is_avatar, id).await?;
        Self::load_updated_user(uid).await
    }

    async fn delete(&self, uid: i64, is_avatar: bool, id: i64) -> anyhow::Result<()> {
        UserHistoryRepo::delete(uid, is_avatar, id).await
    }
}

impl UserHistoryAdapter {
    async fn load_updated_user(uid: i64) -> anyhow::Result<UserInfo> {
        let entity = UserGetRepo::single_find_user_by_id(uid)
            .await?
            .ok_or_else(|| anyhow::anyhow!("用户不存在"))?;
        let info: UserInfo = entity.into();
        if let Err(error) = UserProfileCache::delete_user_info(uid).await {
            warn!(
                "[🤐 ADAPTER] - ❌️ 用户资料缓存失效失败: uid={}, error={}",
                uid, error
            );
        }
        Ok(info)
    }
}

//////// END
