// repo_adapter/src/user/vip/add.rs -- 适配器 - USER - 贵宾 - 发布适配器
// 2026/8/6 12:40 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_user::command::vip::VipCommand;
use port::cola_user::view::add::UserViewAddPort;
use port::cola_user::vip::UserVipPort;
use port::cola_user::vip::add::VipAddPort;
use service::cola_user::vip::add::VipAddService;

////////

/// # [ADD ADAPTER] - 用户贵宾发布适配器
/// * `DESC`: `COLA USER - VIP Add Adapter`
pub struct UserVipAddAdapter;

#[async_trait]
impl VipAddPort for UserVipAddAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 开通 VIP
    async fn add_vip(
        &self,
        uid: i64,     // 操作者 ID
        user_id: i64, // 用户 ID
    ) -> Result<()> {
        let cmd = VipCommand {
            id: user_id,
            vip_type: 1,
            pay_method: 1,
            amount: 0,
            remark: String::new(),
            source: String::from("app"),
        };
        VipAddService::add_vip(uid, user_id, &cmd).await?;
        Ok(())
    }

    ////////

    /// # 2. [ADAPTER] - 取消 VIP
    async fn del_vip(&self, uid: i64, id: i64) -> Result<()> {
        todo!()
    }
}

//////// END
