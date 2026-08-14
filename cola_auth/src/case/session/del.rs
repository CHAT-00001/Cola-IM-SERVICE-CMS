// auth/src/case/session/del.rs
// 🆔 验证中心 - case - 会话用例编排
// 2026/8/14 02:06 Created.

////////

use crate::kits::token::{kit_encrypt_refresh_token, kit_generate_access_token};
use anyhow::{Result, anyhow};
use cola_data::auth::entity::session::AuthSessionEntity;
use cola_data::auth::info::session::AccessTokenInfo;
use service::auth::session::SessionService;

////////

/// # [DEL CASE] - 会话用例
/// * `desc`: `登录会话 用例`
pub struct SessionDelCase;

impl SessionDelCase {
    // 💡

    ////////

    /// # 1. [CASE] - 退出登录
    pub async fn case_logout(uid: i64, session_id: &str, device_id: &str) -> Result<()> {
        // 1. 检查会话ID是否为空
        if session_id.is_empty() {
            return Err(anyhow!("Invalid sync_id"));
        }

        // 2. Call SERVICE ..
        SessionService::del_auth_session_info(session_id, device_id).await?;
        Ok(())
    }
}

////// END
