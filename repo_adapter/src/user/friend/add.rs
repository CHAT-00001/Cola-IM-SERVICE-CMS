// repo_adapter/src/user/friend/add.rs
// 🔌 插头 - 可乐用户 - 朋友 - 添加/移除
// 2026/8/6 解耦: 添加朋友 / 移除朋友

////////

use anyhow::Result;

////////

/// # [ADAPTER] - 添加朋友(或更新存在关系)
pub async fn upsert_friend(
    _uid: i64, // 操作者ID
    _id: i64, // 目标用户ID
) -> Result<()> {
    // 🚧 TODO: 对接 FriendService
    Ok(())
}

/// # [ADAPTER] - 移除朋友
pub async fn del_friend(
    _uid: i64, // 操作者ID
    _id: i64, // 目标用户ID
) -> Result<()> {
    // 🚧 TODO: 对接 FriendService
    Ok(())
}

//////// END