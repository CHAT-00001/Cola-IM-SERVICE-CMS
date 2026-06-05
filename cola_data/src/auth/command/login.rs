// cola_data/src/auth/command/session  -- AUTH - Command - 登录命令
// 2026/4/13 00:49 by wx: cestbon10080

////////

use serde::{Deserialize, Serialize};
use validator::Validate;

////////

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct AuthLoginCommand {
    pub id: i64,
    #[serde(default = "default_area_code")]
    pub area_code: String, // 地区编号
    #[validate(length(min = 7, max = 11, message = "手机号格式不正确"))]
    pub phone_no: String, // 电话号码
    #[validate(length(min = 4, max = 8, message = "验证码长度非法"))]
    pub sms_code: String, // 短信验证码
    pub device_id: String, // 设备ID
    pub platform: String,  // 额外信息，如平台标识等
}

// 默认地区编码 0086 （中国大陆 + 海南岛）
fn default_area_code() -> String {
    "0086".to_string()
}

impl AuthLoginCommand {
    pub fn into_command(self) -> AuthLoginCommand {
        AuthLoginCommand {
            // 注意：AuthLoginCommand 强制要求 id 字段
            // 如果你这是登录接口，id 通常由数据库生成或后期赋值，这里先给个 0
            id: 0,
            area_code: self.area_code,
            phone_no: self.phone_no,
            sms_code: self.sms_code,
            device_id: self.device_id,
            platform: self.platform,
        }
    }
}
