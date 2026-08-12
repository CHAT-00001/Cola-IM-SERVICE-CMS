// auth/src/case/session.rs --
// 🆔 验证中心 - case - 会话用例编排
// 2026-07-19

////////

use crate::kits::token::{kit_encrypt_refresh_token, kit_generate_access_token};
use anyhow::{Result, anyhow};
use cola_data::cola_auth::entity::session::AuthSessionEntity;
use cola_data::cola_auth::info::session::AccessTokenInfo;
use service::cola_auth::session::SessionService;

////////

/// # [CASE] - 会话用例
pub struct SessionCase;

impl SessionCase {
    // 💡

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
    /// 客户端传 raw refresh_token，AES 加密后匹配数据库密文
    pub async fn case_auth_check_session(raw_token: &str) -> Result<AuthSessionEntity> {
        let encrypted = kit_encrypt_refresh_token(raw_token)
            .map_err(|_| anyhow!("Token encrypt failed"))?;
        let session_opt = SessionService::check_auth_session_info(&encrypted).await?;
        session_opt.ok_or_else(|| anyhow!("Session invalid or expired"))
    }

    ////////

    /// 2. [APP USE CASE] - 刷新 Token (续期)
    /// 校验 refresh_token → 生成新 JWT → 返回
    pub async fn case_refresh(
        refresh_token: &str,
    ) -> Result<AccessTokenInfo, anyhow::Error> {
        // 1. AES 加密后再查库，匹配密文
        let encrypted = kit_encrypt_refresh_token(refresh_token)?;
        let session = SessionService::check_auth_session_info(&encrypted)
            .await?
            .ok_or_else(|| anyhow!("刷新令牌无效或已过期"))?;

        // 2. 刷新：生成新的 Access Token（真实 JWT）
        let (access_token, exp_ts) = kit_generate_access_token(session.user_id, &session.device_id)?;

        let new_expiry = chrono::DateTime::from_timestamp(exp_ts, 0)
            .unwrap_or_else(|| chrono::Utc::now() + chrono::Duration::hours(2));

        Ok(AccessTokenInfo {
            access_token,
            access_expired_at: new_expiry,
        })
    }
}

////// END