// port/src/user/vip/stat.rs -- 端口 - USER - 贵宾 - 统计端口
// 2026/8/9 20:17 Created.

////////

use async_trait::async_trait;
use cola_data::cola_user::info::config::UserConfigInfo;
use cola_data::cola_user::info::user::UserInfo;
////////

/// # [STAT PORTS] - 用户贵宾统计端口
/// * `desc`: `COLA USER - VIP Stat Ports`
#[async_trait]
pub trait VipStatPort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 全部贵宾数量
    async fn stat_total_count(&self, user_id: i64) -> anyhow::Result<(u64)>;

    ////////

    /// # 2. [PORT] - 用户的贵宾记录数量
    async fn stat_count_by_user_id(&self, uid: i64, user_id: i64) -> anyhow::Result<(u64)>;
}

//////// END
