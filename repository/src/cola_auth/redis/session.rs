// repository/src/cola_auth/redis/session.rs  -- 仓储中心 - AUTH - redis - session 缓存
// 2026/8/2 改写：Redis 旁路缓存，支持多设备登录

////////

use anyhow::{anyhow, Result};
use cola_data::cola_auth::entity::session::AuthSessionEntity;
use redis::AsyncCommands;

////////

/// # [CACHE] - 会话缓存
pub struct SessionCache;

impl SessionCache {

    ////////

    /// # 1. [CACHE] - 按 access_token 读取会话缓存
    /// * key: `cola_auth:session:token:{access_token}`
    /// * 返回: JSON 序列化的 AuthSessionEntity
    pub async fn get_token_cache(access_token: &str) -> Result<Option<AuthSessionEntity>> {
        let key = format!("cola_auth:session:token:{}", access_token);

        let db = app_config::GLOBAL_DB
            .get()
            .ok_or_else(|| anyhow!("GLOBAL_DB not initialized"))?;
        let mut conn = db.redis_conn.clone();

        let json: Option<String> = conn.get(&key).await?;
        match json {
            Some(v) => {
                let entity: AuthSessionEntity = serde_json::from_str(&v)
                    .map_err(|e| anyhow!("SessionCache: JSON deserialize failed: {}", e))?;
                Ok(Some(entity))
            }
            None => Ok(None),
        }
    }

    ////////

    /// # 2. [CACHE] - 写入会话缓存
    /// * key: `cola_auth:session:token:{access_token}`
    /// * TTL: 动态计算（token 剩余有效期，最小 300 秒）
    pub async fn set_token_cache(access_token: &str, entity: &AuthSessionEntity) -> Result<()> {
        let key = format!("cola_auth:session:token:{}", access_token);

        let now = chrono::Utc::now().timestamp();
        let ttl = if entity.access_expires_at > now {
            (entity.access_expires_at - now).max(300)
        } else {
            300
        };

        let json = serde_json::to_string(entity)
            .map_err(|e| anyhow!("SessionCache: JSON serialize failed: {}", e))?;

        let db = app_config::GLOBAL_DB
            .get()
            .ok_or_else(|| anyhow!("GLOBAL_DB not initialized"))?;
        let mut conn = db.redis_conn.clone();

        let _: () = conn.set_ex(&key, &json, ttl as u64).await?;

        Ok(())
    }

    ////////

    /// # 3. [CACHE] - 删除 token 缓存（注销/过期时调用）
    pub async fn del_token_cache(access_token: &str) -> Result<()> {
        let key = format!("cola_auth:session:token:{}", access_token);

        let db = app_config::GLOBAL_DB
            .get()
            .ok_or_else(|| anyhow!("GLOBAL_DB not initialized"))?;
        let mut conn = db.redis_conn.clone();

        let _: () = conn.del(&key).await?;

        Ok(())
    }

    ////////

    /// # 4. [CACHE] - 登记用户设备（多设备支持）
    /// * key: `cola_auth:session:uid:{uid}:devices`
    /// * value: Set<device_id> (该用户当前所有在线设备)
    pub async fn add_user_device(uid: i64, device_id: &str) -> Result<()> {
        let key = format!("cola_auth:session:uid:{}:devices", uid);

        let db = app_config::GLOBAL_DB
            .get()
            .ok_or_else(|| anyhow!("GLOBAL_DB not initialized"))?;
        let mut conn = db.redis_conn.clone();

        let _: () = conn.sadd(&key, device_id).await?;

        Ok(())
    }

    ////////

    /// # 5. [CACHE] - 获取用户所有在线设备列表
    pub async fn get_user_devices(uid: i64) -> Result<Vec<String>> {
        let key = format!("cola_auth:session:uid:{}:devices", uid);

        let db = app_config::GLOBAL_DB
            .get()
            .ok_or_else(|| anyhow!("GLOBAL_DB not initialized"))?;
        let mut conn = db.redis_conn.clone();

        let devices: Vec<String> = conn.smembers(&key).await?;
        Ok(devices)
    }

    ////////

    /// # 6. [CACHE] - 移除用户设备登记（注销/下线）
    pub async fn remove_user_device(uid: i64, device_id: &str) -> Result<()> {
        let key = format!("cola_auth:session:uid:{}:devices", uid);

        let db = app_config::GLOBAL_DB
            .get()
            .ok_or_else(|| anyhow!("GLOBAL_DB not initialized"))?;
        let mut conn = db.redis_conn.clone();

        let _: () = conn.srem(&key, device_id).await?;
        Ok(())
    }
}

//////// END