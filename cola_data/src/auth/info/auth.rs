// cola_data/src/auth/info/auth  -- 可乐数据中心 - AUTH - Info - 验证壳信息 构造器
// 2026/4/16 07:49

////////

use crate::auth::command::phone::PhoneLoginCommand;
use serde::{Deserialize, Serialize};
use validator::Validate;

////////

/// # 无状态Auth结构体
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AuthContext {
    pub uid: i64,      // 核心：当前登录用户 ID
    pub roles: Vec<String>,    // 扩展：权限角色
    pub device_id: String,     // 安全：设备指纹
    pub is_anonymous: bool,    // 状态：是否为游客
    pub access_token: String,  // 访问令牌
    pub refresh_token: String, // 刷新令牌
}

impl AuthContext {
    //

    ////////

    /// #[BUILDER] 构造函数 - 生成 AuthContext 实例
    pub fn new(
        uid: i64,
        access_token: String,
        refresh_token: String,
        device_id: String,
    ) -> Self {
        // 自动判断是否匿名：没有 uid 就是匿名游客
        let is_anonymous = false;

        Self {
            uid,
            roles: Vec::new(), // 默认空角色，后续可扩展
            device_id,
            is_anonymous,
            access_token,
            refresh_token,
        }
    }
}

/// # ENTITY - 登录体
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthLogin {
    pub user_id: i64,        // 客户端ID
    pub area_code: String,   // 地区编码（默认 0086 ）
    pub phone_no: String,    // 电话号码
    pub sms_code: String,    // 短信验证码
    pub key: Option<String>, // 客户端key
    pub device_id: String,   // 设备ID
    pub platform: String,    // 平台
}

impl AuthLogin {
    // 注意：这里是你的 Request Schema 结构体
    pub fn into_command(self) -> PhoneLoginCommand {
        PhoneLoginCommand {
            area_code: self.area_code,
            phone_no: self.phone_no,
            code: self.sms_code,
            device_id: self.device_id,
            platform: self.platform,
            client_ip: String::new(), // 预留，网关处提取客户端 IP 后注入
        }
    }
}
