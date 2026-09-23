// port/src/cola_user/profile/history.rs -- 用户资料历史端口
// 2026/9/23 Created.

////////

use cola_data::app::page::PageInfo;
use cola_data::cola_user::command::user::update::UpdateUserCommand;
use cola_data::cola_user::info::user::UserInfo;
use cola_data::cola_user::info::user_history::UserHistoryInfo;

////////

/// # [PORT] - 用户资料与历史操作端口
/// * `desc`: `通过 UID 隔离资料修改及历史读写`
#[async_trait::async_trait]
pub trait UserHistoryPort: Send + Sync + 'static {
    async fn update_profile(&self, uid: i64, cmd: UpdateUserCommand) -> anyhow::Result<UserInfo>;
    async fn list(
        &self,
        uid: i64,
        is_avatar: bool,
        page: i64,
        qty: i64,
    ) -> anyhow::Result<(Vec<UserHistoryInfo>, PageInfo)>;
    async fn activate(&self, uid: i64, is_avatar: bool, id: i64) -> anyhow::Result<UserInfo>;
    async fn delete(&self, uid: i64, is_avatar: bool, id: i64) -> anyhow::Result<()>;
}

//////// END
