// /config.rs  -- 用户 配置 端口
// 2026/6/11 02:12

////////


use async_trait::async_trait;
use crate::user::info::config::UserConfigInfo;

////////
#[async_trait]
pub trait ConfigPort : Send + Sync + 'static {
    ////////

    /// # [PORT] - 用户配置
    async fn get_config(
        &self,
        uid: i64,
    ) -> anyhow::Result<(UserConfigInfo)>;
}