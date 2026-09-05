// repo_adapter/src/user/vip/stat.rs -- 适配器层 - USER - 贵宾 - 统计适配器
// 2026/8/10 05:33 Created.

////////

use async_trait::async_trait;
use port::cola_user::vip::stat::VipStatPort;

////////

/// # [STAT ADAPTER] - 计数
/// * `desc`: `COLA USER - Vip Stat Adapter.`
pub struct UserVipStatAdapter;
#[async_trait]
impl VipStatPort for UserVipStatAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 全部贵宾数量
    async fn stat_total_count(&self, user_id: i64) -> anyhow::Result<(u64)> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 用户的充值记录
    async fn stat_count_by_user_id(&self, uid: i64, user_id: i64) -> anyhow::Result<(u64)> {
        todo!()
    }
}

//////// END
