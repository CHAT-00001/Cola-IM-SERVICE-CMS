// cola_auth/src/case/port.rs  -- 可乐验证中心 - 用例层 - 会话用例编排
// 2026/6/9 08:16

////////

use crate::model::vo::session::{SessionResponse, SessionVo};
use anyhow::{Result, anyhow};
use cola_data::auth::command::phone::PhoneLoginCommand;
use cola_data::auth::command::session::SessionCommand;
use cola_data::user::info::user::UserInfo;
use repo::auth::service::session::SessionService;
use repo::auth::service::sms::SmsService;
use repo::user::service::state::UserStateService;
use repo::user::service::user::UserService;

////////

/// # [CASE] - 登录用例
/// * 登录编排 Orchestration
pub struct LoginCase;

// 构造函数
impl LoginCase {

    ////////

    /// # 1. [APP USE CASE] - 手机验证码登录
    /// * `phone_no` 手机号码
    /// * `code` 验证码
    /// # 1. [APP USE CASE] - 手机验证码登录
    pub async fn case_phone(cmd: &PhoneLoginCommand) -> Result<SessionResponse> {
        // 1. 拼接手机号
        let phone = format!("{} + {}", cmd.area_code, cmd.phone_no);

        // 2. 短信验证码校验
        let is_valid = SmsService::verify_sms_code(&phone, &cmd.sms_code).await?;

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

        // --- 修复部分 ---
        // 6. 在调用 Service 之前，先利用 session_cmd 的引用或 clone 进行转换
        // 如果 SessionInfo 实现了 From<&SessionCommand>，可以直接使用 &session_cmd
        // 如果没有，且 SessionCommand 实现了 Clone，建议使用 clone().into()
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
}

//////// END
