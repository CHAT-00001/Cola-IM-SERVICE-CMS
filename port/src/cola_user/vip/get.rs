// port/src/user/vip/get.rs -- 端口 - USER - 贵宾 - 获取端口
// 2026/8/6 00:35 Created.

////////

use async_trait::async_trait;
use cola_data::cola_user::info::vip::UserVipRuleInfo;

////////

/// # [ADD PORTS] - 用户贵宾获取端口
/// * `desc`: `COLA USER - VIP Get Ports`
#[async_trait]
pub trait VipGetPort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 贵宾规则
    async fn get_vip_rule(
        &self,
        uid: i64,    // 作业员
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<UserVipRuleInfo>)>;
}

//////// END
