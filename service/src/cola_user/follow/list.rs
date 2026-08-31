// service/src/user/follow/list.rs
// 👤 服务 - 🗣 可乐用户 - 关注 - 列表
// 2026/8/6 Created.

////////

use anyhow::{Result, anyhow};
use cola_data::cola_user::entity::user::UserEntity;
use cola_data::cola_user::info::user::UserInfo;
use repository::user::pg::follow::list::UserFollowListRepo;

////////

/// # [LIST SERVICE] - 列表
/// * `desc`: `可乐用户 - 关注列表服务`
pub struct FollowListService;

impl FollowListService {
    ////////

    /// # 1. [SERVICE] - 获取我关注的用户Info列表
    pub async fn get_my_follow_infos(uid: i64, offset: i64, limit: i64) -> Result<Vec<UserInfo>> {
        // 1. 查询关注的用户IDs
        let ids = UserFollowListRepo::pg_find_my_follow_ids(uid, limit, offset)
            .await
            .map_err(|e| anyhow!("[🤐 FOLLOW LIST SERVICE]: ❌️ 查询关注IDs失败: {}", e))?;

        if ids.is_empty() {
            tracing::info!("[🗣️ FOLLOW LIST SERVICE]: ✅️ 我的关注列表为空, uid={}", uid);
            return Ok(vec![]);
        }

        // 2. 批量查询用户实体，转换为UserInfo
        let entities = UserFollowListRepo::pg_find_users_by_ids(&ids)
            .await
            .map_err(|e| anyhow!("[🤐 FOLLOW LIST SERVICE]: ❌️ 批量查询用户实体失败: {}", e))?;

        let infos: Vec<UserInfo> = entities.iter().map(|e| Self::entity_to_info(e)).collect();

        tracing::info!(
            "[🗣️ FOLLOW LIST SERVICE]: ✅️ 我的关注列表查询成功, uid={}, count={}",
            uid,
            infos.len()
        );
        Ok(infos)
    }

    ////////

    /// # 2. [SERVICE] - 获取TA关注的用户Info列表
    pub async fn get_he_follow_infos(
        target_id: i64,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<UserInfo>> {
        // 1. 查询TA关注的用户IDs
        let ids = UserFollowListRepo::pg_find_he_follow_ids(target_id, limit, offset)
            .await
            .map_err(|e| anyhow!("[🤐 FOLLOW LIST SERVICE]: ❌️ 查询TA关注IDs失败: {}", e))?;

        if ids.is_empty() {
            tracing::info!(
                "[🗣️ FOLLOW LIST SERVICE]: ✅️ TA的关注列表为空, target_id={}",
                target_id
            );
            return Ok(vec![]);
        }

        // 2. 批量查询用户实体，转换为UserInfo
        let entities = UserFollowListRepo::pg_find_users_by_ids(&ids)
            .await
            .map_err(|e| anyhow!("[🤐 FOLLOW LIST SERVICE]: ❌️ 批量查询用户实体失败: {}", e))?;

        let infos: Vec<UserInfo> = entities.iter().map(|e| Self::entity_to_info(e)).collect();

        tracing::info!(
            "[🗣️ FOLLOW LIST SERVICE]: ✅️ TA的关注列表查询成功, target_id={}, count={}",
            target_id,
            infos.len()
        );
        Ok(infos)
    }

    ////////

    /// # 3. [SERVICE] - 获取关注总数
    pub async fn get_follow_count(uid: i64) -> Result<i64> {
        let count = UserFollowListRepo::pg_count_follows(uid)
            .await
            .map_err(|e| anyhow!("[🤐 FOLLOW LIST SERVICE]: ❌️ 查询关注总数失败: {}", e))?;

        tracing::info!(
            "[🗣️ FOLLOW LIST SERVICE]: ✅️ 关注总数查询成功, uid={}, count={}",
            uid,
            count
        );
        Ok(count)
    }

    ////////

    /// # [HELPER] - Entity → UserInfo 转换(缓存友好)
    fn entity_to_info(e: &UserEntity) -> UserInfo {
        UserInfo {
            id: e.id,
            nickname: e.user_nickname.clone().unwrap_or_default(),
            avatar_url: e.avatar.clone().unwrap_or_default(),
            bg_img: e.bg_img.clone().unwrap_or_default(),
            signature: e.signature.clone(),
            birthday: e.birthday.map(|b| b as i32),
            add_time: e.create_time,
            status: e.status.unwrap_or(0),
            age: None,
            is_following: false, // 动态组装，后续 CASE 层填充
            is_online: false,    // 动态组装
            is_streaming: false, // 动态组装
        }
    }
}

//////// END
