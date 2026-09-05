// repo_adapter/src/user/ban/del.rs -- 适配器 - USER - 浏览 - 检查适配器
// 2026/8/7 05:56 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_user::view::check::UserViewCheckPort;

////////

/// # [CHECK ADAPTER] - 检查适配器
/// * `desc`: `COLA USER - View Check Adapter`
pub struct UserViewCheckAdapter;

// 构造实现
#[async_trait]
impl UserViewCheckPort for UserViewCheckAdapter {
    //

    ////////

    /// # 1. [SERVICE] - 是否看过
    /// * `desc`: `单个软删除`
    async fn is_visited(&self, uid: i64, id: i64) -> Result<(bool)> {
        todo!()
    }
}

//////// END
