// repo_adapter/src/user/config.rs
// 2026-06-12

use async_trait::async_trait;
use cola_data::user::port::config::ConfigPort;
use cola_data::user::info::config::UserConfigInfo;

pub struct UserConfigPortAdapter;

#[async_trait]
impl ConfigPort for UserConfigPortAdapter {
    async fn get_config(&self, _uid: i64) -> anyhow::Result<UserConfigInfo> {
        Err(anyhow::anyhow!("not implemented"))
    }
}