// repository/src/user/redis/profile/mod.rs -- 仓储 - USER - Redis - 用户资料缓存
// 2026/9/5 Created.

////////

use anyhow::{Result, anyhow};
use cola_data::cola_user::info::user::UserInfo;
use redis::AsyncCommands;

////////

const USER_PROFILE_CACHE_TTL: u64 = 48 * 60 * 60;

/// # [REPOSITORY] - 用户资料 Redis 缓存
/// * `desc`: `用户资料缓存读写，TTL 固定为 48 小时`
pub struct UserProfileCache;

impl UserProfileCache {
    ////////

    /// # 1. [REPOSITORY] - 读取用户资料缓存
    /// * `user_id`: 用户 ID
    pub async fn get_user_info(user_id: i64) -> Result<Option<UserInfo>> {
        let key = format!("user:profile:{}", user_id);
        let db = app_config::GLOBAL_DB
            .get()
            .ok_or_else(|| anyhow!("GLOBAL_DB 未初始化"))?;
        let mut conn = db.redis_conn.clone();
        let value: Option<String> = conn.get(key).await?;

        value
            .map(|json| serde_json::from_str(&json).map_err(Into::into))
            .transpose()
    }

    ////////

    /// # 2. [REPOSITORY] - 写入用户资料缓存
    /// * `user_info`: 用户资料
    pub async fn set_user_info(user_info: &UserInfo) -> Result<()> {
        let key = format!("user:profile:{}", user_info.id);
        let value = serde_json::to_string(user_info)?;
        let db = app_config::GLOBAL_DB
            .get()
            .ok_or_else(|| anyhow!("GLOBAL_DB 未初始化"))?;
        let mut conn = db.redis_conn.clone();
        let _: () = conn.set_ex(key, value, USER_PROFILE_CACHE_TTL).await?;
        Ok(())
    }

    ////////
}

//////// END
