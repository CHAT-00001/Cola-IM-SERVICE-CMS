// repo_adapter/src/user/friend/manage.rs
// 🔌 插头 - 可乐用户 - 朋友 - 管理
// 2026/8/6 解耦: 管理操作(封禁/删除)

////////

use anyhow::Result;

////////

/// # [ADAPTER] - 单个删除朋友
pub async fn single_del(
    _uid: i64, // 操作者ID
    _id: i64, // 目标用户ID
) -> Result<u16> {
    // 🚧 TODO: 对接 FriendService
    Ok(0)
}

/// # [ADAPTER] - 批量删除朋友
pub async fn batch_del(
    _uid: i64, // 操作者ID
    _ids: Vec<i64>, // 目标用户IDs
) -> Result<u16> {
    // 🚧 TODO: 对接 FriendService
    Ok(0)
}

//////// END