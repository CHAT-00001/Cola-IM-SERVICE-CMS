// repo_adapter/src/user/vip/add.rs
// 🔌 适配器 - 可乐用户 - 贵宾 - 开通/取消
// 2026/8/6 12:40 Created.

////////

use anyhow::Result;
use cola_data::cola_user::command::vip::VipCommand;
use service::cola_user::vip::add::VipAddService;

////////

/// # [ADAPTER] - 开通 VIP
pub async fn add_vip(uid: i64, target_id: i64) -> Result<()> {
    let cmd = VipCommand {
        id: target_id,
        vip_type: 1,
        pay_method: 1,
        amount: 0,
        remark: String::new(),
        source: String::from("app"),
    };
    VipAddService::add_vip(uid, target_id, &cmd).await?;
    Ok(())
}

/// # [ADAPTER] - 取消 VIP
pub async fn cancel_vip(uid: i64, target_id: i64) -> Result<()> {
    VipAddService::cancel_vip(uid, target_id).await?;
    Ok(())
}

//////// END
