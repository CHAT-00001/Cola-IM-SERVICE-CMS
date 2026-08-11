// /manage.rs
//
// 2026/8/10 23:27 Created.

////////

use async_trait::async_trait;
use cola_data::cola_user::info::black::UserBlackInfo;
use port::cola_user::black::manage::UserBlackManagePort;

/// # [MANAGE ADAPTER] - 管理
/// * `desc`: `USER - 用户角色管理适配器`
pub struct UserRoleManageAdapter;
#[async_trait]
impl UserBlackManagePort for UserRoleManageAdapter {
    async fn get_black_list(
        &self,
        _uid: i64,
        _target_id: i64,
        _limit: i64,
    ) -> anyhow::Result<Vec<UserBlackInfo>> {
        Ok(vec![])
    }
}
