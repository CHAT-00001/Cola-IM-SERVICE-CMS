// cola_data/src/auth/info/auth  -- 数据 - AUTH - Info - 验证壳信息 构造器
// 2026/4/16 07:49

////////

use crate::auth::command::phone::PhoneLoginCommand;
use serde::{Deserialize, Serialize};
use validator::Validate;

////////

/// # [AUTH] - 无状态Auth结构体
#[derive(Debug, Clone, Default, Serialize, Deserialize, Validate)]
pub struct AuthContext {
    pub uid: i64,               // 核心：当前登录用户 ID
    pub iam_roles: Vec<String>, // 扩展：权限角色
    pub device_id: String,      // 安全：设备指纹
    pub is_anonymous: bool,     // 状态：是否为游客
    pub access_token: String,   // 访问令牌
    pub refresh_token: String,  // 刷新令牌
}

impl AuthContext {
    // 💡

    ////////

    /// #[BUILDER] 构造函数 - 生成 AuthContext 实例
    pub fn new(uid: i64, access_token: String, refresh_token: String, device_id: String) -> Self {
        // 自动判断是否匿名：没有 uid 就是匿名游客
        let is_anonymous = false;

        Self {
            uid,
            iam_roles: Vec::new(), // 默认空角色，后续可扩展
            device_id,
            is_anonymous,
            access_token,
            refresh_token,
        }
    }
}

//////// END