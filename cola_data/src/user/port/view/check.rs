// user/port/black/check.rs
// 用户 - port - 浏览 - 检查
// 2026/8/6 00:44 Created.

////////

use crate::user::info::config::UserConfigInfo;
use crate::user::info::user::UserInfo;
use async_trait::async_trait;

////////

/// # [CHECK PORTS]
/// * `desc`: `浏览检查端口`
#[async_trait]
pub trait ViewCheckPort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 检查
    /// * `desc`: `检查是否已经看过`
    async fn is_visited(
        &self,
        uid: i64, // 操作者ID
        id: i64,  // 目标用户ID
    ) -> anyhow::Result<(bool)>;
}

//////// END
