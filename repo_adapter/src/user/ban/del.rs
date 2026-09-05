// repo_adapter/src/user/ban/del.rs -- 适配器 - USER - 封禁 - 删除适配器
// 2026/8/7 05:56 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_user::ban::del::UserBanDelPort;
////////

/// # [DEL ADAPTER] - 用户封禁删除适配器
/// * `desc`: `COLA USER - Ban Delete Adapter`
pub struct BanDelAdapter;

// 构造实现
#[async_trait]
impl UserBanDelPort for BanDelAdapter {
    //

    ////////

    /// # 1. [SERVICE] - 单个
    /// * `desc`: `单个软删除`
    async fn single_soft_del(&self, uid: i64, id: i64) -> Result<(u16)> {
        todo!()
    }

    /// # 2. [SERVICE] - 批量
    /// * `desc`: `批量软删除`
    async fn batch_soft_del(&self, uid: i64, ids: Vec<i64>) -> Result<(u16)> {
        todo!()
    }
}

//////// END
