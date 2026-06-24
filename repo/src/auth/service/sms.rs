// repo/src/auth/service/sms.rs  -- 仓储中心 - AUTH - Service - SMS
// 2026/06/05 09:00

////////

use anyhow::{anyhow, Result};
use crate::auth::redis::sms::SmsCache; // 引入刚才写的 Cache 层

////////

/// # [SERVICE] - 短信服务
pub struct SmsService;


// 构造函数
impl SmsService {

    ////////

    /// # [SERVICE] - 频率检查
    /// 增加这个接口，case 层调用它来判断是否允许发送
    pub async fn check_send_frequency(phone: &str) -> Result<bool> {
        let now = chrono::Utc::now().timestamp();

        // 1. 获取上次发送时间
        if let Some(last_time) = SmsCache::get_last_send_time(phone).await? {
            // 2. 核心逻辑：如果当前时间 - 上次发送时间 < 60秒，则拒绝
            if now - last_time < 60 {
                return Ok(false);
            }
        }

        // 3. 如果允许发送，顺手更新一下时间戳 (这里是个小优化，防止并发)
        SmsCache::set_last_send_time(phone, now).await?;
        Ok(true)
    }

    ////////

    /// # [SERVICE] - 存储验证码
    /// 封装好 TTL，Biz 层调用时无需关心过期时间
    pub async fn store_sms_code(phone: &str, code: &str) -> Result<()> {
        // 默认 300 秒有效期
        SmsCache::set_sms_code(phone, code, 300).await
    }

    ////////

    /// # [SERVICE] - 校验手机短信验证码 (核心业务)
    /// * params: phone / code
    pub async fn verify_sms_code(phone: &str, code: &str) -> Result<bool> {
        let cached_code = SmsCache::get_sms_code(phone).await?;
        match cached_code {
            Some(c) if c == code => Ok(true),
            _ => Ok(false),
        }
    }

    ////////

    /// # [SERVICE] - 校验邮箱验证码 (核心业务)
    /// * params: phone / code
    pub async fn verify_email_code( email: &str, code: &str) -> Result<bool> {
        let cached_code = SmsCache::get_sms_code(email).await?;
        match cached_code {
            Some(c) if c == code => Ok(true),
            _ => Ok(false),
        }
    }

    ////////

    /// # [SERVICE] - 消费/失效验证码
    /// 校验成功后调用此接口删除缓存，防止重放攻击
    pub async fn consume_sms_code(phone: &str) -> Result<()> {
        // 调用 Cache 层删除 key (你需要去 SmsCache 加一个 del 接口)
        Ok(())
    }
}

//////// END