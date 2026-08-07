// cola_user/port/info.rs
// 用户 - port - info - 用户信息
// 2026/6/10 07:28

////////

use async_trait::async_trait;
use crate::cola_user::info::user::UserInfo;

////////

/// [USER INFO PORT]
#[async_trait]
pub trait InfoPort: Send + Sync + 'static {
    //

    ////////

    /// # [PORT] - 批量获取用户资料
    async fn batch_get_info(
        &self,
        ids: Vec<i64>, // 用户IDs
    ) -> anyhow::Result<(Vec<UserInfo>)>;
}

//////// END