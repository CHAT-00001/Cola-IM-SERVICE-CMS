// repo_adapter/src/fs/session/mod.rs -- 🔌 适配器 - FS - SESSION - mod
// 2026/8/8 Created.

////////

use async_trait::async_trait;
use port::auth::session::{AuthSessionPort, SessionPort, SessionUserInfo, SessionVerifyVo};
use port::cola_video::comment::VideoCommentPort;
use service::auth::session::SessionService;
use std::sync::Arc;
////////

pub mod add; // 发布
pub mod alive; // 存活
pub mod check; // 检查
pub mod del; // 删除
pub mod get; // 获取
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 统计

////////

/// # [BUILD] - 构建 SESSION Port
/// * `desc`: 验证会话端口构造器
pub fn build_auth_session_port() -> AuthSessionPort {
    AuthSessionPort {
        add: Arc::new(add::SessionAddAdapter),
        check: Arc::new(check::SessionCheckAdapter),
        del: Arc::new(del::SessionDelAdapter),
        get: Arc::new(get::SessionGetAdapter),
        list: Arc::new(list::SessionListAdapter),
        manage: Arc::new(manage::SessionManageAdapter),
        stat: Arc::new(stat::SessionStatAdapter),
    }
}

//////// END

////////

/// # [SESSION ADAPTER] - 会话校验适配器
/// * `desc`: `登录会话`
pub struct SessionPortAdapter;

#[async_trait]
impl SessionPort for SessionPortAdapter {
    //

    ////////

    /// # 1. [🔌 ADAPTER] - 获取会话
    /// * 机制：SessionService::check_auth_session_info → cache-first (Redis → PG) → 组装 SessionVerifyVo
    async fn get_session(&self, token: &str) -> anyhow::Result<Option<SessionVerifyVo>> {
        tracing::info!(
            "[🗣️ ADAPTER]:  ✅️ get_session 请求, token_len={}, token_preview={}...",
            token.len(),
            &token.chars().take(20).collect::<String>(),
        );

        if token.is_empty() {
            tracing::warn!("[😭 ADAPTER]: ⚠️ token 为空，返回 None");
            return Ok(None);
        }

        // 1. 调用 SessionService 查询会话 (cache-first)
        let session_opt = SessionService::check_auth_session_info(token)
            .await
            .map_err(|e| {
                tracing::error!("[🤐 ADAPTER]: ❌️ 查询会话失败: {}", e);
                anyhow::anyhow!("[🤐 ADAPTER]: ❌️ 查询会话失败: {}", e)
            })?;

        // 2. 将 AuthSessionEntity 转换为 SessionVerifyVo
        match session_opt {
            Some(session) => {
                tracing::info!(
                    "[ADAPTER]: ✅ session 查询成功，user_id={}, device_id={}, status={}",
                    session.user_id,
                    session.device_id,
                    session.status,
                );
                Ok(Some(SessionVerifyVo {
                    user_info: SessionUserInfo {
                        uid: session.user_id,
                        roles: vec![], // 预留：后续从权限表加载
                        status: session.status,
                    },
                }))
            }
            None => {
                tracing::warn!(
                    "[😭 ADAPTER]: ⚠️ session 查询未命中，access_token 在 DB/Redis 中不存在或已过期"
                );
                Ok(None)
            }
        }
    }
}

//////// END
