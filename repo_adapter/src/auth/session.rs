// repo_adapter/src/auth/session.rs
// 适配器 - AUTH - 会话校验
// 2026/8/2 重构：使用 SessionService(cache-first + PG) 而非直接解码 JWT + UserRepo

////////

use async_trait::async_trait;
use cola_data::auth::port::session::{SessionPort, SessionUserInfo, SessionVerifyVo};
use repository::auth::service::session::SessionService;

////////

/// # [ADAPTER] - 会话校验适配器
/// * `desc`: 将 SessionService 的 AuthSessionEntity 转换为 SessionPort 要求的 SessionVerifyVo
pub struct SessionPortAdapter;

#[async_trait]
impl SessionPort for SessionPortAdapter {

    ////////

    /// # 1. [🔌 ADAPTER] - 获取会话
    /// * 机制：SessionService::check_auth_session_info → cache-first (Redis → PG) → 组装 SessionVerifyVo
    async fn get_session(&self, token: &str) -> anyhow::Result<Option<SessionVerifyVo>> {
        if token.is_empty() {
            return Ok(None);
        }

        // 1. 调用 SessionService 查询会话 (cache-first)
        let session_opt = SessionService::check_auth_session_info(token)
            .await
            .map_err(|e| anyhow::anyhow!("[ADAPTER]: 查询会话失败: {}", e))?;

        // 2. 将 AuthSessionEntity 转换为 SessionVerifyVo
        match session_opt {
            Some(session) => {
                Ok(Some(SessionVerifyVo {
                    user_info: SessionUserInfo {
                        uid: session.user_id,
                        roles: vec![],  // 预留：后续从权限表加载
                        status: session.status,
                    },
                }))
            }
            None => Ok(None),
        }
    }
}

//////// END