// cola_auth/src/biz/session.rs -- AUTH - 逻辑层 - 完整会话编排
// 2026/06/05 08:00

////////

use crate::model::vo::session::SessionResponse;
use anyhow::{Result, anyhow};
use cola_data::auth::command::session::SessionCommand;
use cola_data::auth::entity::session::AuthSessionEntity;
use cola_data::user::info::user::UserInfo;
use repo::auth::service::session::SessionService;

////////
pub struct SessionLogic;

impl SessionLogic {
    /// # [LOGIC] - 手机验证码登录
    pub async fn logic_auth_login_by_phone(cmd: &SessionCommand) -> Result<SessionResponse> {
        // 1. 验证码校验
        let cached_code = AuthUserRepo::get_sms_code(&cmd.phone_no).await?;
        if cached_code.as_deref() != Some(&cmd.sms_code) {
            return Err(anyhow!("验证码错误或已过期"));
        }

        // 2. 执行用户落地
        let (user_entity, is_new_user) =
            AuthUserRepo::upsert_user_by_phone(&cmd.phone_no, &cmd.sms_code).await?;

        // 3. 使用 SessionCommand 构造函数生成会话载荷
        // 这里生成随机 Token 字符串 (你可以接入 JWT 或 Redis 生成逻辑)
        let session_cmd = SessionCommand::new_with_defaults(
            user_entity.id,
            "mock_access_token_".to_string(),
            "mock_refresh_token_".to_string(),
            cmd.device_id.clone(),
            cmd.platform,
        );

        // 4. 调用 Service 物理落地 (含挤下线逻辑)
        SessionService::save_auth_session_info(session_cmd).await?;

        // 5. 组装响应
        let mut session_res = SessionResponse::default();
        session_res.user_info = UserInfo::from(user_entity);
        session_res.is_new_user = is_new_user;

        Ok(session_res)
    }

    ////////

    /// # [LOGIC] - 退出登录
    pub async fn logic_auth_logout(sync_id: &str) -> Result<()> {
        if sync_id.is_empty() {
            return Err(anyhow!("Invalid sync_id"));
        }
        SessionService::del_auth_session_info(sync_id).await?;
        Ok(())
    }

    ////////

    /// # [LOGIC] - 鉴权检查 (中间件使用)
    pub async fn logic_auth_check_session(token: &str) -> Result<AuthSessionEntity> {
        let session_opt = SessionService::check_auth_session_info(token).await?;
        session_opt.ok_or_else(|| anyhow!("Session invalid or expired"))
    }

    ////////

    /// # [LOGIC] - 刷新 Token (续期)
    /// * 机制：直接传入旧的 SessionCommand，逻辑层负责“生成”新的 SessionCommand 并落地
    pub async fn logic_auth_refresh_token(
        cmd: &SessionCommand,
    ) -> Result<String> {
        // 1. 鉴权：直接查库校验 cmd 里的 access_token 是否还在有效期
        let session = logic_auth_check_session(&cmd.access_token).await?;

        // 2. 刷新：构造一个新的 SessionCommand (保持用户ID和设备信息，更新 Token 时效)
        let new_access_token = "new_token_".to_string(); // 这里对接你的 Token 生成器

        let new_cmd = SessionCommand::new_with_defaults(
            session.user_id,
            new_access_token.clone(),
            session.refresh_token, // 继续使用旧的 Refresh Token
            session.device_id,
            session.platform,
        );

        // 3. 落地：更新会话信息
        SessionService::update_auth_session_info(new_cmd).await?;

        Ok(new_access_token)
    }
}

//////// END
