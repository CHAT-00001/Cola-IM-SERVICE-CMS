// repo_adapter/src/auth/session/check.rs -- 适配器 - AUTH - SESSION - 检查
// 2026/8/9 20:48 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::auth::session::check::SessionCheckPort;
use port::auth::session::{SessionPort, SessionUserInfo, SessionVerifyVo};
use service::auth::session::SessionService;

////////

/// # [CHECK ADAPTER] - 检查
/// * `desc`: `AUTH - 验证会话检查适配器`
#[derive(Debug, Default, Clone)]
pub struct SessionCheckAdapter;

#[async_trait]
impl SessionCheckPort for SessionCheckAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 健康
    async fn check_health(&self, uid: i64, comment_id: i64) -> Result<(bool)> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 状态
    async fn check_state(&self, uid: i64, comment_id: i64) -> Result<(bool)> {
        todo!()
    }

    ////////

    /// # 3. [ADAPTER] - 归属
    async fn is_owner(
        &self,
        uid: i64,
        user_id: i64,    // 用户 ID
        comment_id: i64, // 评论 ID
    ) -> Result<(bool)> {
        todo!()
    }
}

////////

#[async_trait]
impl SessionPort for SessionCheckAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 根据 access_token 获取有效会话
    /// * `desc`: `调用 SessionService 校验 Token，并转换为 SessionVerifyVo`
    async fn get_session(&self, token: &str) -> anyhow::Result<Option<SessionVerifyVo>> {
        if token.trim().is_empty() {
            return Ok(None);
        }

        let session = SessionService::check_auth_session_info(token)
            .await
            .map_err(|error| anyhow::anyhow!("[🤐 ADAPTER]: ❌️ 查询会话失败: {error}"))?;

        Ok(session.map(|session| SessionVerifyVo {
            user_info: SessionUserInfo {
                uid: session.user_id,
                roles: Vec::new(),
                status: session.status,
            },
        }))
    }
}

//////// END
