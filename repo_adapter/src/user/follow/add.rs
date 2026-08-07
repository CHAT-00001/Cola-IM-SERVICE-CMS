// repo_adapter/src/cola_user/follow/add.rs
// 🔌 插头 - 可乐用户 - 关注 - 添加/取消
// 2026/8/6 Created.

////////

use anyhow::Result;
use repository::cola_user::service::follow::add::FollowService;

////////

/// # [ADAPTER] - 关注用户
pub async fn follow(uid: i64, target_id: i64) -> Result<()> {
    let _ = FollowService::follow(uid, target_id).await?;
    Ok(())
}

/// # [ADAPTER] - 取消关注
pub async fn unfollow(uid: i64, target_id: i64) -> Result<()> {
    let _ = FollowService::unfollow(uid, target_id).await?;
    Ok(())
}

/// # [ADAPTER] - 获取关注IDs
pub async fn get_follow_ids(uid: i64, offset: i64, limit: i64) -> Result<Vec<i64>> {
    let ids = FollowService::get_follow_ids(uid, offset, limit).await?;
    Ok(ids)
}

/// # [ADAPTER] - 检查是否已关注
pub async fn check_followed(uid: i64, target_id: i64) -> Result<bool> {
    let ids = FollowService::get_follow_ids(uid, 0, 1).await?;
    Ok(ids.contains(&target_id))
}

//////// END