// repo_adapter/src/cola_user/ban/del.rs
// 🔌 适配器 - 可乐用户 - 封禁 - 删除服务
// 2026/8/7 05:56 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_user::port::view::check::ViewCheckPort;

////////

/// # [DEL SERVICE] - 删除
/// * `desc`: `用户封禁删除服务`
pub struct ViewCheckService;

// 构造实现
#[async_trait]
impl ViewCheckPort for ViewCheckService {
    //

    ////////

    /// # 1. [SERVICE] - 是否看过
    /// * `desc`: `单个软删除`
    async fn is_visited(&self, uid: i64, id: i64) -> Result<(bool)> {
        todo!()
    }
}

//////// END
