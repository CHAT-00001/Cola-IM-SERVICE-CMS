// repository/src/user/service/vip.rs
// 仓储中心 - USER - Service - 贵宾
// 2026/8/6 Created.

////////

use crate::user::pg::vip::add::VipAddRepo;
use anyhow::{Result, anyhow};
use cola_data::user::command::vip::VipCommand;

////////

/// # [VIP SERVICE] - 贵宾服务
pub struct VipService;

impl VipService {

    ////////

    /// # 1. [SERVICE] - 开通 VIP
    /// * `uid` 操作者
    /// * `target_id` 开通目标用户
    /// * `cmd` VIP 充值命令
    pub async fn add_vip(
        uid: i64,
        target_id: i64,
        cmd: &VipCommand,
    ) -> Result<i64> {
        let id = VipAddRepo::pg_save_vip_record(uid, target_id, cmd)
            .await
            .map_err(|e| anyhow!("[VIP SERVICE]: 保存VIP充值记录失败: {}", e))?;

        tracing::info!("[VIP SERVICE]: VIP充值成功, uid={}, target_id={}, record_id={}", uid, target_id, id);
        Ok(id)
    }

    ////////

    /// # 2. [SERVICE] - 取消 VIP
    pub async fn cancel_vip(uid: i64, target_id: i64) -> Result<()> {
        VipAddRepo::pg_cancel_vip_record(uid, target_id)
            .await
            .map_err(|e| anyhow!("[VIP SERVICE]: 取消VIP记录失败: {}", e))?;

        tracing::info!("[VIP SERVICE]: VIP取消成功, uid={}, target_id={}", uid, target_id);
        Ok(())
    }

    ////////

    /// # 3. [SERVICE] - 检查 VIP 状态
    pub async fn check_vip_status(user_id: i64) -> Result<bool> {
        let is_vip = VipAddRepo::pg_check_vip_status(user_id)
            .await
            .map_err(|e| anyhow!("[VIP SERVICE]: 检查VIP状态失败: {}", e))?;

        Ok(is_vip)
    }
}

//////// END