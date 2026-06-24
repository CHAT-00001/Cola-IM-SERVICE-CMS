// cola_auth/src/case/add.rs  -- AUTH - 用例层 - 登录
// 2026/06/05 09:20

use crate::kits::sms::kit_make_auth_sms_content;
use crate::model::vo::session::{SessionResponse, SessionVo};
use anyhow::{Result, anyhow};
use cola_data::auth::command::email::EmailLoginCommand;
use cola_data::auth::command::phone::PhoneLoginCommand;
use cola_data::auth::command::session::SessionCommand;
use cola_data::user::info::user::UserInfo;
use repo::auth::service::session::SessionService;
use repo::auth::service::sms::SmsService;
use repo::user::service::state::UserStateService;
////////

pub struct AuthAddCase;

impl AuthAddCase {
    //
    
    ////////

    /// # 1. [CASE] - 手机验证码登录
    /// * `params`:  area + phone + code
    pub async fn case_sign_in_by_phone(cmd: &PhoneLoginCommand) -> Result<SessionResponse> {
        // 1. 拼接手机号
        let phone = format!("{} + {}", cmd.area_code, cmd.phone_no);

        // 2. 短信验证码校验
        let is_valid = SmsService::verify_sms_code(&phone, &cmd.code).await?;

        if !is_valid {
            return Err(anyhow!("验证码错误或已过期"));
        }

        // 4. 用户处理
        let (user_info, is_new_user) =
            UserStateService::upsert_user_by_phone(cmd.phone_no.clone()).await?;

        // 5. 构造会话载荷
        let session_cmd = SessionCommand::new_with_defaults(
            "mock_access_token_".to_string(),
            "mock_refresh_token_".to_string(),
        );

        // 6. 在调用 Service 之前，先利用 session_cmd 的引用或 clone 进行转换
        let session_info = session_cmd.clone().into();

        // 7. 调用 Service (将原始 session_cmd 传入)
        let uid = 1;
        SessionService::save_auth_session_info(uid, session_cmd).await?;

        // 8. 组装响应 (使用之前转换好的 session_info)
        let session_res = SessionResponse(SessionVo::new(
            session_info,
            UserInfo::from(user_info),
            is_new_user,
        ));

        Ok(session_res)
    }

    ////////

    /// # 2. [CASE] - 邮箱验证码登录
    /// * `params`: email + code
    pub async fn case_sign_in_by_email(cmd: &EmailLoginCommand) -> Result<SessionResponse> {

        // 1. 抽取参数 email
        let email   = cmd.email.clone();

        // 2. 短信验证码校验
        let is_valid = SmsService::verify_email_code(&email, &cmd.code).await?;

        if !is_valid {
            return Err(anyhow!("验证码错误或已过期"));
        }

        // 4. 用户处理
        let (user_info, is_new_user) =
            UserStateService::upsert_user_by_phone(cmd.email.clone()).await?;

        // 5. 构造会话载荷
        let session_cmd = SessionCommand::new_with_defaults(
            "mock_access_token_".to_string(),
            "mock_refresh_token_".to_string(),
        );

        // 6. 在调用 Service 之前，先利用 session_cmd 的引用或 clone 进行转换
        let session_info = session_cmd.clone().into();

        // 7. 调用 Service (将原始 session_cmd 传入)
        let uid = 1;
        SessionService::save_auth_session_info(uid, session_cmd).await?;

        // 8. 组装响应 (使用之前转换好的 session_info)
        let session_res = SessionResponse(SessionVo::new(
            session_info,
            UserInfo::from(user_info),
            is_new_user,
        ));

        Ok(session_res)
    }

    ////////

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
}
