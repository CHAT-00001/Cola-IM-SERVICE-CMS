// cola_auth/src/api/code.rs  -- 可乐验证中心 - 接口层 - 验证码
// 2026/6/22 06:48

//////

use crate::case::session::{SessionCase};
use cola_data::auth::vo::session::SignResponse;
use cola_data::app::api::ApiQuery;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::auth::command::phone::PhoneLoginCommand;
use tracing::log;
use validator::Validate;
use crate::case::code::AuthCodeCase;
use crate::case::login::LoginCase;

//////

/// # [API] - 验证码 接口
pub struct AuthCodeApi;

impl AuthCodeApi {

    ////////

    /// # [API] - 获取手机短信验证码
    pub async fn handler_get_sms_code(phone: &String) -> AppData<String> {
        if phone.is_empty() {
            return AppData::err(4000, "手机号不能为空", None);
        }

        match AuthCodeCase::case_send_sms_code(phone).await {
            Ok(_) => AppData::ok("验证码发送成功".to_string()),
            Err(e) => AppData::err(500, &e.to_string(), None),
        }
    }

    ////////

    /// # 2. [API] - 获取邮箱验证码
    pub async fn handler_get_email_code(email: &String) -> AppData<String> {
        if email.is_empty() {
            return AppData::err(4000, "邮箱不能为空", None);
        }

        match AuthCodeCase::case_send_sms_code(email).await {
            Ok(_) => AppData::ok("验证码发送成功".to_string()),
            Err(e) => AppData::err(500, &e.to_string(), None),
        }
    }

    ////////

    /// # 3. [API] - 获取密码验证码
    pub async fn handler_get_pwd_code(email: &String) -> AppData<String> {
        if email.is_empty() {
            return AppData::err(4000, "账号不能为空", None);
        }

        match AuthCodeCase::case_send_sms_code(email).await {
            Ok(_) => AppData::ok("验证码发送成功".to_string()),
            Err(e) => AppData::err(500, &e.to_string(), None),
        }
    }
    

}

//////// END
