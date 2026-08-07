// repo_adapter/src/user/user/check.rs
// 🔌 适配器 - 用户 - 用户 - 列表服务
// 2026/8/6 04:19 Created.

////////

use async_trait::async_trait;
use cola_data::user::info::user::UserInfo;
use cola_data::user::port::user::check::UserCheckPort;

////////

/// # [CHECK SERVICE] - 检查
/// * `desc`: `用户检查服务`
pub struct UserCheckAdapter;

// 构造实现
#[async_trait]
impl UserCheckPort for UserCheckAdapter {
    //

    ////////

    /// # 1. [SERVICE] - 最新
    /// * `desc`: `保存新用户记录`
    async fn check_health(&self, id: i64) -> anyhow::Result<(UserInfo)> {
        todo!()
    }

    async fn check_state(&self, id: i64) -> anyhow::Result<(UserInfo)> {
        todo!()
    }
}

//////// END
