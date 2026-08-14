// service/src/user/vip/change.rs
// 服务 - 可乐用户 - 贵宾 - 修改
// 2026/8/7 21:39 Created.

////////

use anyhow::{Result, anyhow};
use repository::user::pg::vip::add::VipAddRepo;

////////

/// # [STATE CHANGE SERVICE] - 修改
/// * `desc`: `贵宾状态修改服务`
pub struct VipStateService;

impl VipStateService {
    //

    ////////

    /// # 1. [SERVICE] - 检查 VIP 状态
    pub async fn check_vip_status(user_id: i64) -> Result<bool> {
        let is_vip = VipAddRepo::pg_check_vip_status(user_id)
            .await
            .map_err(|e| anyhow!("[🤐 VIP SERVICE]: ❌️ 修改VIP状态失败: {}", e))?;

        Ok(is_vip)
    }
}

//////// END
