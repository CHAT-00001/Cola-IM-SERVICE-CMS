// auth/src/case/session/state.rs
// 🆔 验证中心 - case - 会话用例编排
// 2026/8/14 02:08 Created.

////////

use crate::kits::token::{kit_encrypt_refresh_token, kit_generate_access_token};
use anyhow::{Result, anyhow};
use cola_data::auth::entity::session::AuthSessionEntity;
use cola_data::auth::info::session::AccessTokenInfo;
use service::auth::session::SessionService;

////////

/// # [STATE CASE] - 会话 状态 用例
/// * `desc`: `登录会话 用例`
pub struct SessionStateCase;

impl SessionStateCase {
    // 💡

    ////////

    /// # 1. [CASE] - 鉴权检查 (中间件使用)
    /// 客端传 raw refresh_token，AES 加密后匹配数据库密文
    pub async fn case_auth_check_session(raw_token: &str) -> Result<AuthSessionEntity> {
        let encrypted =
            kit_encrypt_refresh_token(raw_token).map_err(|_| anyhow!("Token encrypt failed"))?;
        let session_opt = SessionService::check_auth_session_info(&encrypted).await?;
        session_opt.ok_or_else(|| anyhow!("Session invalid or expired"))
    }

    ////////

    /// # 2. [CASE] - 刷新 Token (续期)
    /// 校验 refresh_token → 生成新 JWT → 返回
    pub async fn case_refresh(refresh_token: &str) -> Result<AccessTokenInfo, anyhow::Error> {
        // 1. AES 加密后再查库，匹配密文
        let encrypted = kit_encrypt_refresh_token(refresh_token)?;
        let session = SessionService::check_auth_session_info(&encrypted)
            .await?
            .ok_or_else(|| anyhow!("刷新令牌无效或已过期"))?;

        // 2. 刷新：生成新的 Access Token（真实 JWT）
        let (access_token, exp_ts) =
            kit_generate_access_token(session.user_id, &session.device_id)?;

        let new_expiry = chrono::DateTime::from_timestamp(exp_ts, 0)
            .unwrap_or_else(|| chrono::Utc::now() + chrono::Duration::hours(2));

        Ok(AccessTokenInfo {
            access_token,
            access_expired_at: new_expiry,
        })
    }
}

////// END
