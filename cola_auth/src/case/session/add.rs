// auth/src/case/add.rs
// AUTH - 用例层 - 登录
// 2026/06/05 09:20

////////

use crate::kits::sms::kit_make_auth_sms_content;
use crate::kits::token::kit_build_session_cmd;
use anyhow::{Result, anyhow};
use cola_data::auth::command::email::EmailLoginCommand;
use cola_data::auth::command::phone::PhoneLoginCommand;
use cola_data::auth::info::session::SessionInfo;
use cola_data::auth::vo::session::{SignResponse, SignVo};
use cola_data::cola_user::command::user::add::UserCommand;
use cola_data::cola_user::info::user::UserInfo;
use cola_data::wallet::command::point::WalletPointInitCommand;
use port::app::ctx::AppContext;
use service::auth::session::SessionService;
use service::auth::sms::SmsService;
use service::cola_user::user::state::UserStateService;
use tracing::log;

////////

/// # [ADD CASE] - 验证发布用例
pub struct AuthAddCase;

impl AuthAddCase {
    //

    ////////

    /// # 1. [CASE] - 手机验证码登录
    /// * `params`:  area + phone + code
    pub async fn case_sign_in_by_phone(
        cmd: &PhoneLoginCommand,
        ctx: &AppContext,
    ) -> Result<SignResponse> {
        // 1. 拼接手机号
        let phone = format!("{}{}", cmd.area_code, cmd.phone_no);

        // 2. 短信验证码校验
        let is_valid = SmsService::verify_sms_code(&phone, &cmd.code).await?;

        if !is_valid {
            return Err(anyhow!("验证码错误或已过期"));
        }

        // 3. 验证码校验通过 → 立即消费/失效，防止重放攻击
        SmsService::consume_sms_code(&phone).await?;

        // 4. 先通过 AUTH identity 检查手机号是否已经绑定用户
        let (user_info, is_new_user) = match ctx.auth.identity.find_user_id_by_phone(&phone).await?
        {
            Some(user_id) => {
                let user = ctx.user.profile.get.single_get_info(user_id).await?;
                (user, false)
            }
            None => {
                // 5. 未命中身份时，使用现有 UserAddPort 创建用户主体
                let mut user_cmd = UserCommand::new_with_phone(phone.clone());
                user_cmd.last_login_ip = Some(cmd.client_ip.clone());
                let user_info = ctx.user.profile.add.save_user(user_cmd).await?;

                // 6. 将手机号和新用户ID写入 AUTH identity
                ctx.auth.identity.bind_phone(user_info.id, &phone).await?;

                // 7. 初始化 Wallet POINT 账户；初始赠送积分按当前配置执行
                ctx.wallet
                    .point
                    .init_point_account(WalletPointInitCommand::new(user_info.id, 0))
                    .await?;

                // 8. 初始化 LIVE 用户资料：等级1，经验0
                ctx.live.user.init_live_user(user_info.id).await?;

                (user_info, true)
            }
        };

        // 5. 构造真实 JWT + 随机 refresh_token（带 device_id 支持多端登录）
        let (session_cmd, raw_access_token, raw_refresh_token) =
            kit_build_session_cmd(user_info.id, &cmd.phone_no, "phone", &cmd.device_id)?;

        // 5. 构建响应的认证信息（给客户端原始 refresh_token，非哈希）
        let auth_info = SessionInfo {
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

    /// # 2. [CASE] - 邮箱验证码登录
    /// * `params`: email + code
    pub async fn case_sign_in_by_email(cmd: &EmailLoginCommand) -> Result<SignResponse> {
        // 1. 抽取参数 email
        let email = cmd.email.clone();

        // 2. 验证码校验
        let is_valid = SmsService::verify_email_code(&email, &cmd.code).await?;

        if !is_valid {
            return Err(anyhow!("验证码错误或已过期"));
        }

        // 3. 验证码校验通过 → 立即消费/失效，防止重放攻击
        SmsService::consume_sms_code(&email).await?;

        // 4. 用户处理（邮箱登录暂时无 client_ip 传递，传 None）
        let (user_info, is_new_user) =
            UserStateService::upsert_user_by_phone(cmd.email.clone(), None).await?;

        // 5. 构造真实 JWT + 随机 refresh_token（带 device_id 支持多端登录）
        let (session_cmd, raw_access_token, raw_refresh_token) =
            kit_build_session_cmd(user_info.id, &email, "email", "email_device")?;

        // 5. 构建响应的认证信息
        let auth_info = SessionInfo {
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

    /// # 3. [CASE] - 退出登录（单设备下线）
    /// * `user_id`   用户 ID
    /// * `device_id` 设备唯一标识 — 只下线当前设备，不影响其他端
    /// * 功能：将 user_id + device_id 命中的会话双 token 置为失效（status = 0）
    pub async fn case_sign_out(user_id: i64, device_id: &str) -> Result<String> {
        if device_id.is_empty() || device_id == "未知设备" {
            return Err(anyhow!("设备标识无效，无法退出"));
        }

        let rows = SessionService::logout_device(user_id, device_id).await?;

        if rows == 0 {
            log::warn!("[退出] 用户 {} 设备 {} 未找到活跃会话", user_id, device_id);
            return Ok("已退出（无活跃会话）".to_string());
        }

        log::info!(
            "[退出] 用户 {} 设备 {} 下线成功（影响 {} 条）",
            user_id,
            device_id,
            rows
        );
        Ok("已安全退出".to_string())
    }
}

//////// END
