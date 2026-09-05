// repo_adapter/src/user/ban/del.rs -- 适配器 - USER - 浏览记录 - 删除适配器
// 2026/8/7 05:56 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_user::view::del::UserViewDelPort;

////////

/// # [DELETE ADAPTER] - 用户浏览记录删除适配器
/// * `desc`: `COLA USER - View Record Delete Adapter`
pub struct UserViewDelAdapter;

// 构造实现
#[async_trait]
impl UserViewDelPort for UserViewDelAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 单个
    /// * `desc`: `单个软删除`
    async fn single_soft_del(&self, uid: i64, id: i64) -> Result<(u16)> {
        todo!()
    }

    /// # 2. [ADAPTER] - 批量
    /// * `desc`: `批量软删除`
    async fn batch_soft_del(&self, uid: i64, ids: Vec<i64>) -> Result<(u16)> {
        todo!()
    }
}

//////// END
