// cola_auth/src/case/session.rs -- 可乐验证中心 - 用例层 - 会话用例编排
// 2026/06/05 08:00

//////

use crate::kits::token::kit_generate_access_token;
use anyhow::{Result, anyhow};
use cola_data::auth::entity::session::AuthSessionEntity;
use cola_data::auth::info::session::AccessTokenInfo;
use repo::auth::service::session::SessionService;

//////

/// # [CASE] - 会话用例
pub struct SessionCase;

impl SessionCase {

    ////////

    /// 1. [APP USE CASE] - 退出登录
    pub async fn case_logout(
        uid: i64,
        session_id: &str,
        device_id: &str,
    ) -> Result<()> {
        if session_id.is_empty() {
            return Err(anyhow!("Invalid sync_id"));
        }
        SessionService::del_auth_session_info(session_id, device_id).await?;
        Ok(())
    }

    ////////

    /// [APP USE CASE] - 鉴权检查 (中间件使用)
    pub async fn case_auth_check_session(token: &str) -> Result<AuthSessionEntity> {
        let session_opt = SessionService::check_auth_session_info(token).await?;
        session_opt.ok_or_else(|| anyhow!("Session invalid or expired"))
    }

    ////////

    /// 2. [APP USE CASE] - 刷新 Token (续期)
    /// 校验 refresh_token → 生成新 JWT → 返回
    pub async fn case_refresh(
        refresh_token: &str,
    ) -> Result<AccessTokenInfo, anyhow::Error> {
        // 1. 鉴权：校验 Refresh Token 是否存在且合法
        let session = SessionService::check_auth_session_info(refresh_token)
            .await?
            .ok_or_else(|| anyhow!("刷新令牌无效或已过期"))?;

        // 2. 刷新：生成新的 Access Token（真实 JWT）
        let (access_token, exp_ts) = kit_generate_access_token(session.user_id, &session.device_id)?;

        let new_expiry = chrono::DateTime::from_timestamp(exp_ts, 0)
            .unwrap_or_else(|| chrono::Utc::now() + chrono::Duration::hours(2));

        // 3. 返回新 JWT + 过期时间
        Ok(AccessTokenInfo {
            access_token,
            access_expired_at: new_expiry,
        })
    }
}

//////// END
