// cola_video/src/port/api/session -- 登录应用层
// 2026-03-11 10:56:12

use std::fmt;
use serde::{Deserialize, Serialize}; // 序列化/反序列化，供transport层交互
use tracing::info; // 日志

/// 通用返回结果类型（供transport层使用）
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    /// 状态码 0:成功 非0:失败
    pub code: i32,
    /// 提示信息
    pub msg: String,
    /// 登录成功返回token，失败返回空
    pub data: Option<String>,
}

/// 登录类型枚举（修复Rust枚举语法，补充序列化）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")] // 序列化时转大写（如CODE/WECHAT）
pub enum LoginType {
    #[serde(rename = "0")]
    Code,        // 手机验证码 (默认）
    #[serde(rename = "1")]
    Email,       // 邮箱密码
    #[serde(rename = "2")]
    Account,     // 账号密码
    #[serde(rename = "3")]
    Apple,       // 苹果登录
    #[serde(rename = "4")]
    Wechat,      // 微信登录
    #[serde(rename = "5")]
    Alipay,      // 支付宝登录
    #[serde(rename = "6")]
    Google,      // Google登录
    #[serde(rename = "7")]
    Facebook,    // Facebook
    #[serde(rename = "8")]
    Whatsapp,    // WhatAPP
}

// 为LoginType实现默认值（手机验证码为默认登录方式）
impl Default for LoginType {
    fn default() -> Self {
        LoginType::Code
    }
}

// 为LoginType实现字符串转换（方便日志/调试）
impl fmt::Display for LoginType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoginType::Code => write!(f, "Code(0)"),
            LoginType::Email => write!(f, "Email(1)"),
            LoginType::Account => write!(f, "Account(2)"),
            LoginType::Apple => write!(f, "Apple(3)"),
            LoginType::Wechat => write!(f, "Wechat(4)"),
            LoginType::Alipay => write!(f, "Alipay(5)"),
            LoginType::Google => write!(f, "Google(6)"),
            LoginType::Facebook => write!(f, "Facebook(7)"),
            LoginType::Whatsapp => write!(f, "Whatsapp(8)"),
        }
    }
}

/// 登录参数结构体（补充序列化，添加默认值和校验）
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LoginQuery {
    #[serde(default)]
    pub login_type: LoginType,   // 登录类型（默认手机验证码）
    pub sms_code: Option<String>,  // 短信验证码
    pub sms_secret: Option<String>, // 短信秘钥（可选）
    #[serde(default = "default_code")]
    pub code: String,  // 地区码 默认 +86
    pub phone_number: Option<String>,  // 电话号码
    pub email: Option<String>, // 邮箱
    pub password: Option<String>, // 补充：密码（账号/邮箱登录需要）
    pub third_party_token: Option<String>, // 补充：第三方登录token（微信/苹果等）
}

// 地区码默认值函数
fn default_code() -> String {
    "+86".to_string()
}

/// 登录核心业务逻辑
/// 返回：LoginResponse（供transport层直接返回给前端）
pub async fn app_login(query: LoginQuery) -> LoginResponse {
    // 1. 日志记录请求参数
    info!(
        "接收到登录请求 | 登录类型: {} | 手机号: {:?} | 邮箱: {:?}",
        query.login_type, query.phone_number, query.email
    );

    // 2. 参数合法性校验
    let validate_result = validate_login_params(&query);
    if let Err(msg) = validate_result {
        return LoginResponse {
            code: 1001,
            msg,
            data: None,
        };
    }

    // 3. 根据登录类型分发处理逻辑
    let token = match query.login_type {
        LoginType::Code => handle_sms_login(&query).await,
        LoginType::Email => handle_email_login(&query).await,
        LoginType::Account => handle_account_login(&query).await,
        LoginType::Wechat | LoginType::Apple | LoginType::Alipay => {
            handle_third_party_login(&query).await
        }
        LoginType::Google | LoginType::Facebook | LoginType::Whatsapp => {
            handle_oversea_third_party_login(&query).await
        }
    };

    // 4. 封装返回结果
    match token {
        Ok(token) => LoginResponse {
            code: 0,
            msg: "登录成功".to_string(),
            data: Some(token),
        },
        Err(msg) => LoginResponse {
            code: 1002,
            msg,
            data: None,
        },
    }
}

// ========== 私有辅助函数 ==========

/// 校验登录参数合法性
fn validate_login_params(query: &LoginQuery) -> Result<(), String> {
    match query.login_type {
        // 手机验证码登录：必须传手机号+验证码
        LoginType::Code => {
            if query.phone_number.is_none() {
                return Err("手机号不能为空".to_string());
            }
            if query.sms_code.is_none() {
                return Err("短信验证码不能为空".to_string());
            }
        }
        // 邮箱登录：必须传邮箱+密码
        LoginType::Email => {
            if query.email.is_none() {
                return Err("邮箱不能为空".to_string());
            }
            if query.password.is_none() {
                return Err("密码不能为空".to_string());
            }
        }
        // 账号密码登录：必须传手机号/邮箱 + 密码
        LoginType::Account => {
            if query.phone_number.is_none() && query.email.is_none() {
                return Err("账号（手机号/邮箱）不能为空".to_string());
            }
            if query.password.is_none() {
                return Err("密码不能为空".to_string());
            }
        }
        // 第三方登录：必须传token
        LoginType::Wechat | LoginType::Apple | LoginType::Alipay |
        LoginType::Google | LoginType::Facebook | LoginType::Whatsapp => {
            if query.third_party_token.is_none() {
                return Err(format!("{}登录token不能为空", query.login_type));
            }
        }
    }
    Ok(())
}

/// 处理短信验证码登录
async fn handle_sms_login(query: &LoginQuery) -> Result<String, String> {
    // TODO: 实现短信验证码校验逻辑
    // 1. 校验sms_code是否有效（调用data层查验证码）
    // 2. 校验通过后生成token（jwt/自定义token）
    Ok("mock_sms_token_123456".to_string()) // 模拟返回token
}

/// 处理邮箱密码登录
async fn handle_email_login(query: &LoginQuery) -> Result<String, String> {
    // TODO: 实现邮箱密码校验逻辑
    Ok("mock_email_token_123456".to_string())
}

/// 处理账号密码登录
async fn handle_account_login(query: &LoginQuery) -> Result<String, String> {
    // TODO: 实现账号密码校验逻辑
    Ok("mock_account_token_123456".to_string())
}

/// 处理国内第三方登录（微信/支付宝/苹果）
async fn handle_third_party_login(query: &LoginQuery) -> Result<String, String> {
    // TODO: 调用第三方接口校验token有效性
    Ok(format!("mock_{:?}_token_123456", query.login_type))
}

/// 处理海外第三方登录（Google/Facebook/Whatsapp）
async fn handle_oversea_third_party_login(query: &LoginQuery) -> Result<String, String> {
    // TODO: 调用海外第三方接口校验token
    Ok(format!("mock_{:?}_token_123456", query.login_type))
}