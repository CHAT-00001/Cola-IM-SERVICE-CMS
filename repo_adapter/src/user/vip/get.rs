// repo_adapter/src/user/vip/get.rs -- 适配器 - USER - 贵宾 - 获取适配器
// 2026/8/6 19:20 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_user::info::vip::UserVipRuleInfo;
use port::cola_user::vip::get::VipGetPort;

////////

/// # [GET ADAPTER] - 用户贵宾获取适配器
/// * `DESC`: `COLA USER - VIP Get Adapter`
pub struct UserVipGetAdapter;

#[async_trait]
impl VipGetPort for UserVipGetAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 获取贵宾开通规则
    async fn get_vip_rule(
        &self,
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<UserVipRuleInfo>)> {
        todo!()
    }
}

//////// END
