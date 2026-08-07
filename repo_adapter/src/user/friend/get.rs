// repo_adapter/src/user/friend/get.rs
// 🔌 插头 - 可乐用户 - 朋友 - 获取
// 2026/8/6 解耦: 获取朋友IDs

////////

use anyhow::Result;
use cola_data::user::info::user::UserInfo;

////////

/// # [ADAPTER] - 获取我的朋友的用户IDs
pub async fn get_my_friend_ids(
    _uid: i64, // 操作者ID
    _id: i64, // 目标用户ID
    _limit: i64, // 数量
    _offset: i64, // 页码
) -> Result<Vec<i64>> {
    // 🚧 TODO: 对接 FriendService
    Ok(vec![])
}

/// # [ADAPTER] - 获取TA朋友的用户IDs
pub async fn get_he_friend_ids(
    _uid: i64, // 操作者ID
    _id: i64, // 目标用户ID
    _limit: i64, // 数量
    _offset: i64, // 页码
) -> Result<Vec<i64>> {
    // 🚧 TODO: 对接 FriendService
    Ok(vec![])
}

/// # [ADAPTER] - 获取朋友我的用户IDs
pub async fn get_friend_me_ids(
    _uid: i64, // 操作者ID
    _id: i64, // 目标用户ID
    _limit: i64, // 数量
    _offset: i64, // 页码
) -> Result<Vec<i64>> {
    // 🚧 TODO: 对接 FriendService
    Ok(vec![])
}

/// # [ADAPTER] - 获取朋友TA的用户IDs
pub async fn get_friend_he_ids(
    _uid: i64, // 操作者ID
    _id: i64, // 目标用户ID
    _limit: i64, // 数量
    _offset: i64, // 页码
) -> Result<Vec<i64>> {
    // 🚧 TODO: 对接 FriendService
    Ok(vec![])
}

/// # [ADAPTER] - 获取朋友的信息
pub async fn get_friending(
    _uid: i64, // 操作者ID
) -> Result<UserInfo> {
    // 🚧 TODO: 对接 FriendService
    Err(anyhow::anyhow!("not implemented"))
}

/// # [ADAPTER] - 获取朋友IDs列表
pub async fn get_list(
    _user_id: i64, // 目标用户ID
    _offset: i64, // 页码
    _limit: i64, // 数量
) -> Result<Vec<i64>> {
    // 🚧 TODO: 对接 FriendService
    Ok(vec![])
}

/// # [ADAPTER] - 获取TA的朋友
pub async fn get_here_list(
    _uid: i64, // 操作者ID
    _user_ids: Vec<i64>, // 返回用户IDs
) -> Result<()> {
    // 🚧 TODO: 对接 FriendService
    Ok(())
}

//////// END