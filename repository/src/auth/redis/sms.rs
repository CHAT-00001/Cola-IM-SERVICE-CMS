// repository/src/auth/redis/sms.rs  -- 仓储中心 - AUTH - redis - sms 缓存
// 2026/06/25 改写：去掉伪代码，走真实 Redis 查询（通过 app_config::GLOBAL_DB 全局连接池）

////////

use anyhow::{anyhow, Result};
use redis::AsyncCommands;

////////


/// # [CACHE] - 短信缓存
/// * `desc`: `手机验证码Redis缓存`
pub struct SmsCache;

impl SmsCache {

    ////////

    /// # [CACHE] - 保存短信验证码到 Redis
    /// * `phone`: 手机号
    /// * `code`: 验证码
    /// * `ttl`: 过期时间 (秒)
    pub async fn set_sms_code(phone: &str, code: &str, ttl: i64) -> Result<()> {
        let key = format!("auth:sms:{}", phone);

        // 从全局 GLOBAL_DB 获取 Redis 连接
        let db = app_config::GLOBAL_DB
            .get()
            .ok_or_else(|| anyhow!("GLOBAL_DB not initialized"))?;
        let mut conn = db.redis_conn.clone();

        // SETEX key ttl value
        let _: () = conn.set_ex(&key, code, ttl as u64).await?;

        Ok(())
    }

    ////////

    /// # [CACHE] - 消费/获取短信验证码
    /// * 从 Redis 取出验证码，不会自动删除（需要调用 del 做消费）
    pub async fn get_sms_code(phone: &str) -> Result<Option<String>> {
        let key = format!("auth:sms:{}", phone);

        let db = app_config::GLOBAL_DB
            .get()
            .ok_or_else(|| anyhow!("GLOBAL_DB not initialized"))?;
        let mut conn = db.redis_conn.clone();

        let code: Option<String> = conn.get(&key).await?;

        Ok(code)
    }

    ////////

    /// # [CACHE] - 获取上次发送时间戳（用于频率控制）
    /// * key: `auth:sms:limit:{phone}`
    pub async fn get_last_send_time(phone: &str) -> Result<Option<i64>> {
        let key = format!("auth:sms:limit:{}", phone);

        let db = app_config::GLOBAL_DB
            .get()
            .ok_or_else(|| anyhow!("GLOBAL_DB not initialized"))?;
        let mut conn = db.redis_conn.clone();

        let timestamp: Option<i64> = conn.get(&key).await?;

        Ok(timestamp)
    }

    ////////

    /// # [CACHE] - 记录发送时间戳（60 秒 TTL 自动过期）
    /// * key: `auth:sms:limit:{phone}`
    /// * value: Unix 时间戳
    pub async fn set_last_send_time(phone: &str, timestamp: i64) -> Result<()> {
        let key = format!("auth:sms:limit:{}", phone);

        let db = app_config::GLOBAL_DB
            .get()
            .ok_or_else(|| anyhow!("GLOBAL_DB not initialized"))?;
        let mut conn = db.redis_conn.clone();

        // TTL 设置为 60 秒，Redis 自动清理，不用手动删
        let _: () = conn.set_ex(&key, timestamp, 60).await?;

        Ok(())
    }

    ////////

    /// # [CACHE] - 消费/删除验证码（校验成功后调用，防重放）
    pub async fn del_sms_code(phone: &str) -> Result<()> {
        let key = format!("auth:sms:{}", phone);

        let db = app_config::GLOBAL_DB
            .get()
            .ok_or_else(|| anyhow!("GLOBAL_DB not initialized"))?;
        let mut conn = db.redis_conn.clone();

        let _: () = conn.del(&key).await?;

        Ok(())
    }
}

//////// END
