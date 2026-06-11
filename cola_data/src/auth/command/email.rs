// cola_data/src/auth/command/email.rs  -- 可乐数据中心 - 验证中心 - Command - 邮箱验证码登录
// 2026/6/9 07:39

////////

use validator::Validate;

/// # [COMMAND] - 邮箱验证码登录命令
#[derive(Debug, Validate)]
pub struct EmailLoginCommand {
    pub email: String,     // 邮箱
    pub code: String,      // 验证码
    pub device_id: String, //
    pub platform: i32,     //
}
