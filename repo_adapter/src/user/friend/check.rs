// repo_adapter/src/cola_user/friend/check.rs
// 🔌 插头 - 可乐用户 - 朋友 - 检查
// 2026/8/6 解耦: 是否已是朋友 / 状态检查

////////

use anyhow::Result;

////////

/// # [ADAPTER] - 检查是否已是朋友
pub async fn is_friended(
    _uid: i64, // 操作者ID
    _id: i64, // 目标用户ID
) -> Result<bool> {
    // 🚧 TODO: 对接 FriendService
    Ok(false)
}

/// # [ADAPTER] - 检查朋友关系状态
pub async fn check_state(
    _uid: i64, // 操作者ID
    _user_id: i64, // 目标用户ID
) -> Result<bool> {
    // 🚧 TODO: 对接 FriendService
    Ok(false)
}

//////// END