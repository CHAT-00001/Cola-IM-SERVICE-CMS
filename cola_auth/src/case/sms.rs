// cola_auth/src/case/sms.rs  -- AUTH - 逻辑层 - 发送短信验证码
// 2026/06/05 09:20

use anyhow::{anyhow, Result};
use repo::auth::service::sms::SmsService;
use crate::kits::sms::kit_make_auth_sms_content;

////////

/// # [BIZ] - 发送短信验证码
pub async fn logic_send_sms_code(
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
    println!("--- 模拟发送短信 --- \nTO: {}\nCONTENT: {}\n------------------", phone_no, sms_content);

    // 4. 【持久化】通过 Service 层存入缓存 (由 Service 内部处理 TTL)
    SmsService::store_sms_code(phone_no, &code)
        .await
        .map_err(|e| anyhow!("验证码存储失败: {}", e))?;

    Ok(())
}

/// # [BIZ] - 验证码校验 (补充：方便你在登录接口里调用)
pub async fn logic_verify_sms_code(phone_no: &str, code: &str) -> Result<()> {
    let is_valid = SmsService::verify_sms_code(phone_no, code).await?;
    if !is_valid {
        return Err(anyhow!("验证码错误或已失效"));
    }

    // 校验成功，立即消费/删除验证码
    SmsService::consume_sms_code(phone_no).await?;
    Ok(())
}