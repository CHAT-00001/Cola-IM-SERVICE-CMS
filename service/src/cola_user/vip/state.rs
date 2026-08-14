// service/src/user/vip/state.rs
// 服务 - 可乐用户 - 贵宾 - 状态
// 2026/8/7 21:39 Created.

////////

use anyhow::{Result, anyhow};
use repository::user::pg::vip::add::VipAddRepo;

////////

/// # [STATE CHECK SERVICE] - 检查
/// * `desc`: `贵宾状态检查服务`
pub struct VipStateService;

impl VipStateService {
    //

    ////////

    /// # 1. [SERVICE] - 检查 VIP 状态
    pub async fn check_vip_status(user_id: i64) -> Result<bool> {
        let is_vip = VipAddRepo::pg_check_vip_status(user_id)
            .await
            .map_err(|e| anyhow!("[🤐 VIP SERVICE]: ❌️ 检查VIP状态失败: {}", e))?;

        Ok(is_vip)
    }
}

//////// END
