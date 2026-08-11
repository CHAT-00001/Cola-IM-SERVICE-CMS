// repo_adapter/src/cola_user/ban/del.rs
// 🔌 适配器 - USER - 浏览 - 检查
// 2026/8/7 05:56 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_user::view::check::UserViewCheckPort;

////////

/// # [CHECK ADAPTER] - 删除
/// * `desc`: `USER - 用户主页浏览检查适配器`
pub struct ViewCheckService;

// 构造实现
#[async_trait]
impl UserViewCheckPort for ViewCheckService {
    //

    ////////

    /// # 1. [SERVICE] - 是否看过
    /// * `desc`: `单个软删除`
    async fn is_visited(&self, uid: i64, id: i64) -> Result<(bool)> {
        todo!()
    }
}

//////// END
