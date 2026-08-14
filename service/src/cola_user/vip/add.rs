// service/src/user/vip/add.rs
// 服务 - 可乐用户 - 贵宾 - 发布
// 2026/8/7 21:38 Created.

////////

use anyhow::{Result, anyhow};
use cola_data::cola_user::command::vip::VipCommand;
use repository::user::pg::vip::add::VipAddRepo;

////////

/// # [VIP ADD SERVICE] - 发布
/// * `desc`: `贵宾发布服务`
pub struct VipAddService;

impl VipAddService {
    //

    ////////

    /// # 1. [SERVICE] - 开通 VIP
    /// * `desc`: `用户自助开通贵宾服务`
    pub async fn add_vip(uid: i64, target_id: i64, cmd: &VipCommand) -> Result<i64> {
        let id = VipAddRepo::pg_save_vip_record(uid, target_id, cmd)
            .await
            .map_err(|e| anyhow!("[🤐 VIP SERVICE]: ❌️ 保存VIP充值记录失败: {}", e))?;

        tracing::info!(
            "[🗣️ VIP SERVICE]: ✅️ VIP充值成功, uid={}, target_id={}, record_id={}",
            uid,
            target_id,
            id
        );
        Ok(id)
    }

    ////////

    /// # 2. [SERVICE] - 取消 VIP
    pub async fn cancel_vip(uid: i64, target_id: i64) -> Result<()> {
        VipAddRepo::pg_cancel_vip_record(uid, target_id)
            .await
            .map_err(|e| anyhow!("[🤐 VIP SERVICE]: ❌️ 取消VIP记录失败: {}", e))?;

        tracing::info!(
            "[🗣️ VIP SERVICE]: ✅️ VIP取消成功, uid={}, target_id={}",
            uid,
            target_id
        );
        Ok(())
    }
}

//////// END
