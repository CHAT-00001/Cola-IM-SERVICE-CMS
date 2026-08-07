// repo_adapter/src/user/vip/add.rs
// 适配器 - USER - 贵宾 - 开通/取消
// 2026/8/6 解耦: 开通VIP / 取消VIP

////////

use anyhow::Result;
use cola_data::user::command::vip::VipCommand;
use repository::user::service::vip::VipService;

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
    VipService::add_vip(uid, target_id, &cmd).await?;
    Ok(())
}

/// # [ADAPTER] - 取消 VIP
pub async fn cancel_vip(uid: i64, target_id: i64) -> Result<()> {
    VipService::cancel_vip(uid, target_id).await?;
    Ok(())
}

//////// END