// cola_auth/src/case/port.rs  -- 可乐验证中心 - 用例层 - 会话用例编排
// 2026/6/9 08:16

//////

use crate::kits::token::kit_build_session_cmd;
use crate::model::vo::session::{SignResponse, SignVo};
use anyhow::{Result, anyhow};
use cola_data::auth::command::phone::PhoneLoginCommand;
use cola_data::user::info::user::UserInfo;
use repo::auth::service::session::SessionService;
use repo::auth::service::sms::SmsService;
use repo::user::service::state::UserStateService;

//////

/// # [CASE] - 登录用例
/// * 登录编排 Orchestration
pub struct LoginCase;

impl LoginCase {

    ////////

    /// # 1. [APP USE CASE] - 手机验证码登录
    pub async fn case_phone(cmd: &PhoneLoginCommand) -> Result<SignResponse> {
        // 1. 拼接手机号
        let phone = format!("{} + {}", cmd.area_code, cmd.phone_no);

        // 2. 短信验证码校验
        let is_valid = SmsService::verify_sms_code(&phone, &cmd.code).await?;

        if !is_valid {
            return Err(anyhow!("验证码错误或已过期"));
        }

        // 3. 用户处理（不存在则自动创建）— 返回真实 user_info
        let (user_info, is_new_user) =
            UserStateService::upsert_user_by_phone(cmd.phone_no.clone(), Some(cmd.client_ip.clone())).await?;

        // 4. 构造真实 JWT + 随机 refresh_token（带 device_id 支持多端登录）
        let (session_cmd, raw_access_token, raw_refresh_token) =
            kit_build_session_cmd(user_info.id, &cmd.phone_no, "phone", &cmd.device_id)?;

        // 5. 构建响应用的认证信息（给客户端的原始 refresh_token，非哈希）
        let auth_info = cola_data::auth::info::session::SessionInfo {
            access_token: raw_access_token,
            refresh_token: raw_refresh_token,
            access_expires_at: session_cmd.access_expires_at,
            refresh_expires_at: session_cmd.refresh_expires_at,
        };

        // 6. 入库保存会话（使用真实 uid）
        SessionService::save_auth_session_info(user_info.id, session_cmd).await?;

        // 7. 组装响应
        Ok(SignResponse(SignVo::new(auth_info, user_info, is_new_user)))
    }

    ////////
}

//////// END
