// cola_auth/src/case/session.rs -- 可乐验证中心 - 用例层 - 会话用例编排
// 2026/06/05 08:00

////////

use crate::model::vo::session::SessionResponse;
use anyhow::{Result, anyhow};
use cola_data::auth::command::phone::PhoneLoginCommand;
use cola_data::auth::command::session::SessionCommand;
use cola_data::auth::entity::session::AuthSessionEntity;
use cola_data::auth::info::session::AccessTokenInfo;
use cola_data::user::info::user::UserInfo;
use repo::auth::service::session::SessionService;
use repo::auth::service::sms::SmsService;

////////

/// # [CASE] - 会话用例
pub struct SessionCase;

// 构造函数
impl SessionCase {

    ////////

    /// 1. # [APP USE CASE] - 退出登录
    /// * - session id 和 device id 双重命中
    pub async fn case_logout(
        uid: i64,         // 操作者 ID
        session_id: &str, // 会话 ID
        device_id: &str,  // 设备 ID
    ) -> Result<()> {
        if session_id.is_empty() {
            return Err(anyhow!("Invalid sync_id"));
        }
        SessionService::del_auth_session_info(session_id, device_id).await?;
        Ok(())
    }

    ////////

    /// # [APP USE CASE] - 鉴权检查 (中间件使用)
    pub async fn case_auth_check_session(token: &str) -> Result<AuthSessionEntity> {
        let session_opt = SessionService::check_auth_session_info(token).await?;
        session_opt.ok_or_else(|| anyhow!("Session invalid or expired"))
    }

    ////////

    /// # 2. [APP USE CASE] - 刷新 Token (续期)
    pub async fn case_refresh(
        refresh_token: &str, // 建议改名以增加可读性
    ) -> Result<AccessTokenInfo, anyhow::Error> {
        // 1. 鉴权：校验 Refresh Token 是否存在且合法
        // 注意：这里 check_auth_session_info 返回的是 Option<AuthSessionEntity>
        let session_opt = SessionService::check_auth_session_info(refresh_token).await?;
        let session = session_opt.ok_or_else(|| anyhow!("刷新令牌无效或已过期"))?;

        // 2. 刷新：生成新的 Access Token (假设你这里调用了 Token 生成逻辑)
        let new_access_token = "new_token_123456".to_string();

        // 注意：你需要定义 expiry 时间，这里假设有一个辅助函数
        let new_expiry = chrono::Utc::now() + chrono::Duration::hours(2);

        // 3. 落地：更新会话信息 (可选：如果你的逻辑需要持久化更新)
        // SessionService::update_auth_session_info(...).await?;

        // 4. 返回精简结构
        Ok(AccessTokenInfo {
            access_token: new_access_token,
            access_expired_at: new_expiry,
        })
    }
}

//////// END
