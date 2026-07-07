// repo_adapter/src/user/info.rs
// 2026-06-12

use async_trait::async_trait;
use cola_data::user::port::info::InfoPort;

pub struct UserInfoPortAdapter;

#[async_trait]
impl InfoPort for UserInfoPortAdapter {

    ////////

    /// # [ADAPTER] - 获取用户信息
    async fn get_info(&self, _user_id: i64) -> anyhow::Result<()> {
        Ok(())
    }
}