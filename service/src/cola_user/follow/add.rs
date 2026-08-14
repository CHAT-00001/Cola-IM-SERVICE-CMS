// service/src/user/follow/add.rs
// 👤 服务 - 🗣 可乐用户 - 关注 - 发布(关注/取关)
// 2026/8/3 14:33 Created.

////////

use anyhow::{Result, anyhow};
use repository::user::pg::follow::add::UserFollowAddRepo;

////////

/// # [ADD SERVICE] -
/// * `desc`: `关注 添加 服务`
pub struct FollowService;

impl FollowService {

    ////////

    /// # 1. [SERVICE] - 关注用户
    /// * `uid` 操作者
    /// * `target_id` 目标用户ID
    pub async fn follow(uid: i64, target_id: i64) -> Result<u64> {
        let rows = UserFollowAddRepo::save_follow_record(uid, target_id, String::new(), 1)
            .await
            .map_err(|e| anyhow!("[FOLLOW SERVICE]: 保存关注记录失败: {}", e))?;

        tracing::info!("[FOLLOW SERVICE]: 关注成功, uid={}, target_id={}", uid, target_id);
        Ok(rows)
    }

    ////////

    /// # 2. [SERVICE] - 取消关注
    /// * `uid` 操作者
    /// * `target_id` 目标用户ID
    pub async fn unfollow(uid: i64, target_id: i64) -> Result<u64> {
        let rows = UserFollowAddRepo::save_follow_record(uid, target_id, String::new(), 0)
            .await
            .map_err(|e| anyhow!("[FOLLOW SERVICE]: 取消关注失败: {}", e))?;

        tracing::info!("[FOLLOW SERVICE]: 取消关注成功, uid={}, target_id={}", uid, target_id);
        Ok(rows)
    }

    ////////

    /// # 3. [SERVICE] - 获取关注IDs
    /// * `uid` 操作者
    pub async fn get_follow_ids(uid: i64, offset: i64, limit: i64) -> Result<Vec<i64>> {
        let ids = UserFollowAddRepo::find_follow_ids_by_uid(uid, limit, offset)
            .await
            .map_err(|e| anyhow!("[FOLLOW SERVICE]: 查询关注列表失败: {}", e))?;

        tracing::info!("[FOLLOW SERVICE]: 关注列表查询成功, uid={}, count={}", uid, ids.len());
        Ok(ids)
    }

    ////////

    /// # 4. [SERVICE] - 获取粉丝IDs
    /// * `user_id` 被关注者ID
    pub async fn get_follower_ids(user_id: i64, offset: i64, limit: i64) -> Result<Vec<i64>> {
        let ids = UserFollowAddRepo::find_follower_ids_by_user_id(user_id, limit, offset)
            .await
            .map_err(|e| anyhow!("[FOLLOW SERVICE]: 查询粉丝列表失败: {}", e))?;

        tracing::info!("[FOLLOW SERVICE]: 粉丝列表查询成功, user_id={}, count={}", user_id, ids.len());
        Ok(ids)
    }
}

//////// END