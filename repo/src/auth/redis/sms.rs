// repo/src/auth/cache/sms.rs  -- 仓储中心 - AUTH - redis - sms
// 2026/06/05 08:45 by wx: cestbon10080

////////

use anyhow::{anyhow, Result};
// 假设你项目中有一个全局的 redis 连接池，这里用伪代码表示
// use crate::redis_pool;

////////
pub struct SmsCache;

impl SmsCache {
    /// # [CACHE] - 保存短信缓存
    /// * `phone`: 手机号
    /// * `code`: 验证码
    /// * `ttl`: 过期时间 (秒)
    pub async fn set_sms_code(phone: &str, code: &str, ttl: i64) -> Result<()> {
        // 伪代码实现：SETEX key ttl value
        let key = format!("auth:sms:{}", phone);

        // 实际逻辑：
        // let mut conn = redis_pool().get().await?;
        // conn.set_ex(key, code, ttl as usize).await?;

        println!("[CACHE] Saved SMS: {} -> {} (TTL: {}s)", key, code, ttl);
        Ok(())
    }

    /// # [CACHE] - 消费/获取短信缓存
    /// * 机制：获取并返回，如果需要“消费后立即失效”，可以在下面加个 DEL
    pub async fn get_sms_code(phone: &str) -> Result<Option<String>> {
        let key = format!("auth:sms:{}", phone);

        // 实际逻辑：
        // let mut conn = redis_pool().get().await?;
        // let code: Option<String> = conn.get(key).await?;

        // 模拟返回
        println!("[CACHE] Fetched SMS for: {}", key);
        Ok(Some("123456".to_string()))
    }

    /// # [CACHE] - 检查是否在冷却期
    pub async fn get_last_send_time(phone: &str) -> Result<Option<i64>> {
        // 伪逻辑：GET auth:sms:limit:{phone} -> 返回存储的时间戳
        Ok(Some(1234567890)) // 示例
    }

    /// # [CACHE] - 记录发送时间
    pub async fn set_last_send_time(phone: &str, timestamp: i64) -> Result<()> {
        // 伪逻辑：SETEX auth:sms:limit:{phone} 60 timestamp
        // 💡 关键：TTL 设置为 60 秒，这样 Redis 会自动帮你清理记录
        Ok(())
    }
}

//////// END