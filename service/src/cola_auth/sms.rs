// service/src/cola_auth/sms.rs
// 服务 - 可乐验证 - 短信 - 模块
// 2026/6/5 09:00 Created.

////////

use anyhow::{Result, anyhow};
use repository::cola_auth::redis::sms::SmsCache;
use tracing::log;

////////

/// # [SMS SERVICE] - 短信
/// * `desc`: `短信发送服务`
pub struct SmsService;

// 构造实现
impl SmsService {
    //

    ////////

    /// # 1. [SERVICE] - 频率检查
    /// `desc`: `增加这个接口，case 层调用它来判断是否允许发送`
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

    /// # 2. [SERVICE] - 存储验证码
    /// `desc`:`封装好 TTL，Biz 层调用时无需关心过期时间`
    pub async fn store_sms_code(
        phone: &str, // 手机号码
        code: &str,  // 验证码
    ) -> Result<()> {
        // 默认 300 秒有效期
        SmsCache::set_sms_code(phone, code, 300).await
    }

    ////////

    /// # 3. [SERVICE] - 校验手机短信验证码 (核心业务)
    /// * `params`: `phone + code`
    pub async fn verify_sms_code(
        phone: &str, // 电话号码
        code: &str,  // 验证码
    ) -> Result<bool> {
        // 打印当前正在查询的 Key 和传入的验证码
        log::info!("正在校验 Key: {}, 传入验证码: {}", phone, code);

        let cached_code = SmsCache::get_sms_code(phone).await?;

        match cached_code {
            Some(c) => {
                log::info!(
                    "[😊 Redis] -  ✅️ 查到的值: '{}', 匹配结果: {}",
                    c,
                    c == code
                );
                Ok(c == code)
            }
            None => {
                log::warn!("[😭 Redis] - ❌️ 中未找到该 Key 的缓存");
                Ok(false)
            }
        }
    }

    ////////

    /// # 4. [SERVICE] - 校验邮箱验证码 (核心业务)
    /// * `params`: `email + code`
    pub async fn verify_email_code(
        email: &str, // 邮箱
        code: &str,  // 验证码
    ) -> Result<bool> {
        let cached_code = SmsCache::get_sms_code(email).await?;
        match cached_code {
            Some(c) if c == code => Ok(true),
            _ => Ok(false),
        }
    }

    ////////

    /// # 5. [SERVICE] - 消费/失效验证码
    /// `desc`: `校验成功后调用此接口删除缓存，防止重放攻击`
    pub async fn consume_sms_code(phone: &str, // 电话号码
    ) -> Result<()> {
        SmsCache::del_sms_code(phone).await
    }
}

//////// END
