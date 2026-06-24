// cola_auth/src/api/code.rs  -- 可乐验证中心 - 接口层 - 验证码
// 2026/6/22 06:48

////////

use crate::case::session::{SessionCase};
use crate::model::vo::session::SessionResponse;
use cola_data::app::api::ApiQuery;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::auth::command::phone::PhoneLoginCommand;
use tracing::log;
use validator::Validate;
use crate::case::code::AuthCodeCase;
use crate::case::login::LoginCase;

////////

/// # [API] - 验证码 接口
pub struct AuthCodeApi;

// 构造函数
impl AuthCodeApi {

    ////////

    /// # [API] - 获取手机短信验证码
    /// * `params`: phone_no
    pub async fn handler_get_sms_code(phone: &String) -> AppData<String> {
        // 1. 基础校验
        if phone.is_empty() {
            return AppData::err(400, "手机号不能为空", None);
        }

        // 2. 🚀 剥离 Port，静态下钻到本地的验证码 Biz
        match AuthCodeCase::case_send_sms_code(phone).await {
            Ok(_) => AppData::ok("验证码发送成功".to_string()),
            Err(e) => AppData::err(500, &e.to_string(), None),
        }
    }

    ////////

    /// # 2. [API] - 获取邮箱验证码
    /// * `params`: email
    pub async fn handler_get_email_code(email: &String) -> AppData<String> {
        // 1. 基础校验
        if email.is_empty() {
            return AppData::err(400, "邮箱不能为空", None);
        }

        // 2. 🚀 剥离 Port，静态下钻到本地的验证码 Biz
        match AuthCodeCase::case_send_sms_code(email).await {
            Ok(_) => AppData::ok("验证码发送成功".to_string()),
            Err(e) => AppData::err(500, &e.to_string(), None),
        }
    }
    

}

//////// END


