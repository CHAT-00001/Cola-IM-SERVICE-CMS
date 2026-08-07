// cola_auth/src/case/code.rs  -- AUTH - case - 会话
// 2026/6/22 07:00

////////

use anyhow::{Result, anyhow};
use service::cola_auth::sms::SmsService;
use crate::kits::sms::kit_make_auth_sms_content;

////////

/// # [USE CASE] - 验证码 用例
/// * 登录编排 Orchestration
pub struct AuthCodeCase;

// 构造函数
impl AuthCodeCase {
    ////////

    /// # 1. [CASE] - 发送短信验证码
    pub async fn case_send_sms_code(
        phone_no: &str,
    ) -> Result<()> {

        // 1. 【防轰炸】通过 Service 层检查频率
        let is_allowed = SmsService::check_send_frequency(phone_no)
            .await
            .map_err(|e| anyhow!("频率检查失败: {}", e))?;

        if !is_allowed {
            return Err(anyhow!("发送验证码太频繁，请稍后再试"));
        }

        // 2. 生成验证码
        let (code, sms_content) = kit_make_auth_sms_content();

        // 3. 【发送】调用外部网关
        println!("[CASE]: --- 模拟发送短信 --- \nTO: {}\nCONTENT: {}\n------------------", phone_no, sms_content);

        // 4. 【持久化】通过 Service 层存入缓存 (由 Service 内部处理 TTL)
        SmsService::store_sms_code(phone_no, &code)
            .await
            .map_err(|e| anyhow!("验证码存储失败: {}", e))?;

        Ok(())
    }
}

//////// END
