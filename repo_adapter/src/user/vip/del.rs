// repo_adapter/src/user/vip/del.rs -- 适配器 - USER - 贵宾 - 删除适配器
// 2026/8/6 解耦: VIP删除操作

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_user::vip::del::VipDelPort;

////////

/// # [DELETE ADAPTER] - 用户贵宾删除适配器
/// * `DESC`: `COLA USER - VIP Delete Adapter`
pub struct UserVipDelAdapter;

#[async_trait]
impl VipDelPort for UserVipDelAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 单个删除
    async fn single_delete(&self, uid: i64, id: i64) -> Result<(u64)> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 批量删除
    async fn batch_delete(&self, uid: i64, ids: Vec<i64>) -> Result<(u64)> {
        todo!()
    }
}

//////// END
