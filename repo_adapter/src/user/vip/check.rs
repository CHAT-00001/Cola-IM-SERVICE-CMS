// repo_adapter/src/user/vip/check.rs -- 适配器 - USER - 贵宾 - 检查
// 2026/8/6 14:20 Created.

////////

use anyhow::Result;
use service::cola_user::vip::state::VipStateService;

////////

/// # [ADAPTER] - 检查VIP状态
pub async fn check_state(
    _uid: i64,    // 操作者ID
    user_id: i64, // 目标用户ID
) -> Result<bool> {
    VipStateService::check_vip_status(user_id).await
}

//////// END
