// cola_data/src/auth/command/phone.rs  -- 可乐数据中心 - AUTH - Command - 手机短信验证码登录验证器
// 2026/4/13 00:49 by wx: cestbon10080

////////

use serde::{Deserialize, Serialize};
use validator::Validate;

////////

/// # [COMMAND] - 手机验证码登录
#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct PhoneLoginCommand {
    #[serde(default = "default_area_code")]
    pub area_code: String, // 地区编号

    #[validate(length(min = 7, max = 15, message = "手机号格式不正确"))] // 国际化通常支持到15位
    pub phone_no: String, // 电话号码

    #[validate(length(min = 4, max = 8, message = "验证码长度非法"))]
    pub sms_code: String, // 短信验证码

    pub device_id: String, // 设备唯一标识 (用于多设备会话管理)
    pub platform: String,  // 平台标识，如: "ios", "android", "web"
}

// 默认地区编码 0086
fn default_area_code() -> String {
    "0086".to_string()
}

impl PhoneLoginCommand {
    /// 快速校验接口
    /// 在 Case 层直接调用此方法，确保输入参数符合业务规则
    pub fn validate_params(&self) -> Result<(), validator::ValidationErrors> {
        self.validate()
    }

    /// 辅助方法：生成一个脱敏的日志记录，保护用户隐私
    /// 在记录登录日志时，不要打印完整的手机号或验证码
    pub fn to_log_info(&self) -> String {
        format!("platform: {}, device: {}, phone: {}***{}",
                self.platform,
                self.device_id,
                &self.phone_no[..3],
                &self.phone_no[self.phone_no.len()-2..]
        )
    }
}