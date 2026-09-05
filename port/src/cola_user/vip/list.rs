// port/src/user/vip/list.rs -- 端口 - USER - 贵宾 - 列表端口
// 2026/8/6 00:35 Created.

////////

use async_trait::async_trait;
use cola_data::cola_user::info::vip::UserVipInfo;

////////

/// # [LIST PORTS] - 用户贵宾列表端口
/// * `desc`: `COLA USER - VIP List Ports`
#[async_trait]
pub trait VipListPort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 最新的
    async fn get_new_list(
        &self,
        uid: i64,    // 操作员 ID
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<UserVipInfo>)>;

    ////////

    /// # 2. [PORT] - 用户的
    async fn get_user_list(
        &self,
        uid: i64,     // 操作员 ID
        user_id: i64, // 目标用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<(Vec<UserVipInfo>)>;
}

//////// END
