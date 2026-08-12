// repository/src/auth/pg/session/ban
// 仓储 - AUTH - pg - session - 会话校验服务(cache-first + pg fallback + 多设备支持)
// 2026/8/2 09:43

////////

use crate::cola_auth::pg::session::SessionRepo;
use crate::cola_auth::redis::session::SessionCache;
use anyhow::{Result, anyhow};
use cola_data::cola_auth::entity::session::AuthSessionEntity;

////////

/// # [SERVICE] - 会话状态检查服务
pub struct SessionStateService;

impl SessionStateService {

    ////////

    /// # 1. [CACHE-FIRST] - 检查会话(按 access_token)
    /// * 1️⃣ 先查 Redis 缓存
    /// * 2️⃣ 缓存 Miss → 查 PG
    /// * 3️⃣ PG 命中 → 回填 Redis(多设备登记)
    /// * 4️⃣ PG 也未命中 → None
    pub async fn check_session(access_token: &str) -> Result<Option<AuthSessionEntity>> {
        if access_token.is_empty() {
            return Ok(None);
        }

        // 1️⃣ 先查 Redis
        if let Ok(Some(cached)) = SessionCache::get_token_cache(access_token).await {
            tracing::debug!("🔐 SessionState: Redis 命中");
            return Ok(Some(cached));
        }

        // 2️⃣ 缓存 Miss：查 PG
        let session_opt = SessionRepo::find_active_by_access_token(access_token)
            .await
            .map_err(|e| anyhow!("SessionState: PG 查询失败: {}", e))?;

        // 3️⃣ 回填 Redis + 登记设备
        if let Some(ref session) = session_opt {
            // 回填 token 缓存
            if let Err(e) = SessionCache::set_token_cache(access_token, session).await {
                tracing::warn!("SessionState: 回填 Redis 失败: {}", e);
            }
            // 登记用户设备(多设备支持)
            if let Err(e) = SessionCache::add_user_device(session.user_id, &session.device_id).await {
                tracing::warn!("SessionState: 登记设备失败: {}", e);
            }
        }

        Ok(session_opt)
    }

    ////////

    /// # 2. [SERVICE] - 检查某用户是否在指定设备登录
    /// * 多设备比对：先去 PG 查该设备是否有活跃 session
    pub async fn check_device_active(
        user_id: i64,
        device_id: &str,
    ) -> Result<bool> {
        // Redis 快速检查(Set 成员判断)
        if let Ok(devices) = SessionCache::get_user_devices(user_id).await {
            if devices.iter().any(|d| d == device_id) {
                return Ok(true);
            }
        }

        // Redis 未命中或不含该设备：查 PG 确认
        let sessions = SessionRepo::find_online_devices_by_uid(user_id)
            .await
            .map_err(|e| anyhow!("SessionState: 查询在线设备失败: {}", e))?;

        let found = sessions.iter().any(|s| s.device_id == device_id);
        Ok(found)
    }

    ////////

    /// # 3. [SERVICE] - 登出时清理缓存(双删策略)
    pub async fn invalidate_session(
        access_token: &str,
        uid: i64,
        device_id: &str,
    ) -> Result<()> {
        // 删除 token 缓存
        if let Err(e) = SessionCache::del_token_cache(access_token).await {
            tracing::warn!("SessionState: 删除缓存失败: {}", e);
        }
        // 移除设备登记
        if let Err(e) = SessionCache::remove_user_device(uid, device_id).await {
            tracing::warn!("SessionState: 移除设备登记失败: {}", e);
        }
        Ok(())
    }
}

//////// END